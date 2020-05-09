import React, { createContext, useState, useEffect, useContext } from 'react';
import { useEffectQueue } from 'unanimity/hooks';
import { KIND, ORDER, kinds, orders, api, printerr, trace } from "unanimity/lib";
import { isEqual, remove, without } from 'lodash';
import {clean} from "../lib";


export const PostsChange    = 0b00000001;
export const TagsChange     = 0b00000010;
export const KeywordChange  = 0b00000100;
export const KindChange     = 0b00001000;
export const OrderChange    = 0b00010000;

export const StreamDiff = Object.freeze({
  PostsChange,
  TagsChange,
  KeywordChange,
  KindChange,
  OrderChange
});

function streamDifference(prev, next) {
  let diff = 0;

  if (prev === null)
    return 0b00011111;

  if (!isEqual(prev.posts, next.posts))
    diff |= PostsChange;
  if (!isEqual(prev.tags, next.tags))
    diff |= TagsChange;
  if (!isEqual(prev.keywords, next.keywords))
    diff |= KeywordChange;
  if (!isEqual(prev.kind, next.kind))
    diff |= KindChange;
  if (!isEqual(prev.order, next.order))
    diff |= OrderChange;

  return diff;
}

const StreamContext = createContext(null, streamDifference);

const query = state => ({
  kind: state.kind.value.key,
  order: state.order.value,
  tags: state.tags.value,
  keywords: state.keywords.value
});

export function StreamProvider({ children }) {

  const pushEffect = useEffectQueue();

  // Don't move the methods ouf the state, as this is a context, any change to
  // the values given to children will provoke a rerender, the best way to
  // avoid that is to allow react to execute its diff algorithm.
  const [state, setState] = useState({

    posts: {
      value: [],
      of(id) {
        const prefetch = this.value.filter(p => p.id === id);
        if (prefetch.length)
          return Promise.resolve(prefetch[0]);
        return api.posts.of(id);
      },
      add(post) {
        const promise = api.posts.add(post);
        pushEffect([
          promise,
          post => setState(s => ({
            ...state,
            posts: { ...this, value: [...this.value, post] }
          })) || post,
          printerr // TODO
        ]);
        return promise;
      },
      remove(post) {
        const promise = api.posts.delete(post.id);
        pushEffect([
          promise,
          () => setState(s => ({
            ...s,
            posts: {
              ...this,
              value: remove(s.posts.value, p => p.id !== post.id)
            }
          })),
          printerr // TODO
        ]);
        return promise;
      },
      vote(post, vote) {
        const promise = api.posts.vote(post.id, vote);
        pushEffect([
          promise,
          post => setState(s => ({
            ...s,
            posts: {
              ...this,
              value: s.posts.value.map(p => p.id === post.id ? post : p)
            }
          })) || post,
          printerr // TODO
        ]);
        return promise;
      },
      comment(comment) {
        trace('TODO - COMMENT');
        return Promise.resolve(comment);
      },
      flag(post) {
        trace('TODO - FLAG');
        return Promise.resolve(post);
      },
      hide(post) {
        trace('TODO - HIDE');
        return Promise.resolve(post);
      }
    },

    kind: {
      available: kinds,
      value: KIND.ALL,
      set(kind) {
        if (this.value === kind)
          return;
        setState(s => ({ ...s, kind: { ...this, value: kind } }));
      }
    },

    order: {
      available: orders,
      value: ORDER.RANK.DESC,
      set(order) {
        setState(s => ({ ...s, order: { ...this, value: order } }));
      },
    },

    tags: {
      available: [],
      value: [],
      add(tag) {
        if (this.value.includes(tag))
          return;
        const tags = [...state.tags.value, tag];
        setState(s => ({ ...s, tags: { ...s.tags, value: tags } }));
      },
      remove(tag) {
        if (!this.value.includes(tag))
          return;
        const tags = without(this.value, tag);
        setState(s => ({ ...s, tags: { ...s.tags, value: tags } }));
      },
      set(tag) {
        const tags = (tag instanceof Array) ? tag : [tag];
        setState(s => ({ ...s, tags: { ...s.tags, value: tags } }));
      }
    },

    keywords: {
      value: [],
      add(kw) {
        if (this.value.includes(kw))
          return;
        const keywords = [ ...state.keywords.value, kw ];
        setState(s => ({ ...s, keywords: { ...s.keywords, value: keywords } }));
      },
      remove(kw) {
        if (!this.value.includes(kw))
          return;
        const keywords = without(state.keywords.value, kw);
        setState(s => ({ ...s, keywords: { ...s.keywords, value: keywords } }));
      }
    }

  });

  useEffect(() => pushEffect([
    api.posts.where(clean(query(state), true)),
    posts => setState(s => ({
      ...s,
      posts: { ...s.posts, value: posts }
    })),
    printerr // TODO
  ]), [state.kind.value, state.order.value, state.tags.value, state.keywords.value]);

  /* Get the tags on first mount */
  useEffect(() => pushEffect([
    api.tags(),
    ({ tags }) => setState(state => ({ ...state, tags: { ...state.tags, available: tags } })),
    printerr // TODO
  ]), []);

  return (
    <StreamContext.Provider value={state}>
      {children}
    </StreamContext.Provider>
  );

}

export const useStream = () => useContext(StreamContext);