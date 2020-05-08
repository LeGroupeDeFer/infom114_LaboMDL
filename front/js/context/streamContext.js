import React, { createContext, useState, useEffect, useContext } from 'react';
import { useEffectQueue } from 'unanimity/hooks';
import { KIND, ORDER, kinds, orders, api, printerr, trace } from "unanimity/lib";
import { isEqual, remove, without } from 'lodash';


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
      add(post) {
        const promise = api.posts.add(post);
        pushEffect([
          promise,
          post => setState(state => ({
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
          () => setState(state => ({
            ...state,
            posts: { ...this, value: remove(this.value, p => p.id === post.id) }
          })),
          printerr // TODO
        ]);
      }
    },

    kind: {
      available: kinds,
      value: KIND.ALL,
      set(kind) {
        if (this.value === kind)
          return;
        setState(state => ({ ...state, kind: { ...this, value: kind } }));
      }
    },

    order: {
      available: orders,
      value: ORDER.RANK.DESC,
      set(order) {
        if (this.value === order)
          return;
        setState(state => ({ ...state, order: { ...this, value: order } }));
      },
    },

    tags: {
      available: [],
      value: [],
      add(tag) {
        if (this.value.includes(tag))
          return;
        const tags = [...this.value, tag];
        setState(state => ({ ...state, tags: { ...this, value: tags } }));
      },
      remove(tag) {
        if (!this.value.includes(tag))
          return;
        const tags = without(this.value, tag);
        trace(tags);
        setState(state => ({ ...state, tags: { ...this, value: tags } }));
      },
    },

    keywords: {
      value: [],
      add(kw) {
        if (this.value.includes(kw))
          return;
        const keywords = [ ...this.value, kw ];
        setState(state => ({ ...state, keywords: { ...this, value: keywords } }));
      },
      remove(kw) {
        if (!this.value.includes(kw))
          return;
        const keywords = without(this.value, kw);
        setState(state => ({ ...state, keywords: { ...this, value: keywords } }));
      }
    }
  });

  useEffect(() => pushEffect([
    api.posts.where(query(state)),
    posts => setState(state => ({
      ...state,
      posts: { ...state.posts, value: posts }
    })),
    printerr // TODO
  ]), [state.kind.value, state.order.value, state.tags.value, state.keywords.value]);

  /* Get the tags on first mount */
  useEffect(() => pushEffect([
    api.tags(),
    ({ tags }) => setState(state => ({ ...state, tags: { ...state.tags, available: tags } })),
    printerr // TODO
  ]),
  []
  );

  return (
    <StreamContext.Provider value={state}>
      {children}
    </StreamContext.Provider>
  );

}

export const useStream = () => useContext(StreamContext);