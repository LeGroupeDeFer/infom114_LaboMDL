import React, { createContext, useState, useEffect, useContext } from 'react';
import { useEffectQueue } from 'unanimity/hooks';
import {
  KIND,
  ORDER,
  kinds,
  orders,
  api,
  printerr,
  trace,
} from 'unanimity/lib';
import isEqual from 'lodash/isEqual';
import remove from 'lodash/remove';
import without from 'lodash/without';
import { clean } from '../lib';

export const PostsChange = 0b00000001;
export const TagsChange = 0b00000010;
export const KeywordChange = 0b00000100;
export const KindChange = 0b00001000;
export const OrderChange = 0b00010000;

export const StreamDiff = Object.freeze({
  PostsChange,
  TagsChange,
  KeywordChange,
  KindChange,
  OrderChange,
});

function streamDifference(prev, next) {
  let diff = 0;

  if (prev === null) return 0b00011111;

  if (!isEqual(prev.posts, next.posts)) diff |= PostsChange;
  if (!isEqual(prev.tags, next.tags)) diff |= TagsChange;
  if (!isEqual(prev.keywords, next.keywords)) diff |= KeywordChange;
  if (!isEqual(prev.kind, next.kind)) diff |= KindChange;
  if (!isEqual(prev.order, next.order)) diff |= OrderChange;

  return diff;
}

const StreamContext = createContext(null, streamDifference);

const query = (state) => ({
  kind: state.kind.value.key,
  order: state.order.value,
  tags: state.tags.value,
  keywords: state.keywords.value,
});

export function StreamProvider({ children }) {
  const pushEffect = useEffectQueue();

  // Don't move the methods ouf the state, as this is a context, any change to
  // the values given to children will provoke a rerender, the best way to
  // avoid that is to allow react to execute its diff algorithm.
  const [state, setState] = useState({
    posts: {
      focus: null,
      value: [],
      _updatePost(promise) {
        const that = this;

        pushEffect([
          promise,
          (post) => setState((s) => {

            const currentPosts = s.posts.value;
            let updatedPosts;
            if (s.posts.value.some(p => p.id === post.id))
              updatedPosts = currentPosts.map(p => (p.id === post.id ? post : p));
            else
              updatedPosts = [ ...currentPosts, post ];

            return { ...s, posts: { ...s.posts, value: updatedPosts } };
          }) || post,
          printerr, // TODO
        ]);
        return promise;
      },

      of(id) {
        const prefetch = this.value.filter((p) => Number(p.id) === Number(id));
        let promise;
        if (prefetch.length) {
          promise = Promise.resolve(prefetch[0]);
        } else {
          promise = api.posts.of(id);
        }

        return this._updatePost(promise.then((post) => {
          if (post.kind === 'poll') {
            return api.posts.pollData(id).then((pollData) => {
              post.answers = pollData.answers;
              post.userAnswer = pollData.userAnswer;
              return post;
            });
          }
          return post;
        }));
      },

      add(post) {
        const promise = api.posts.add(post);
        pushEffect([
          promise,
          (post) =>
            setState((s) => ({
              ...state,
              posts: { ...this, value: [...this.value, post] },
            })) || post,
          printerr, // TODO
        ]);
        return promise;
      },
      remove(post) {
        const promise = api.posts.delete(post.id);
        pushEffect([
          promise,
          () =>
            setState((s) => ({
              ...s,
              posts: {
                ...this,
                value: remove(s.posts.value, (p) => p.id !== post.id),
              },
            })),
          printerr, // TODO
        ]);
        return promise;
      },
      comment(post, comment) {
        trace('TODO - COMMENT');
        return Promise.resolve(comment);
      },
      vote(post, vote) {
        return this._updatePost(api.posts.vote(post.id, vote));
      },
      flag(post, reason, cancel) {
        return this._updatePost(api.posts.flag(post.id, reason, cancel));
      },
      hide(post) {
        return this._updatePost(api.posts.hide(post.id));
      },
      lock(post) {
        return this._updatePost(api.posts.lock(post.id));
      },
      watch(id, payload) {
        return this._updatePost(api.posts.watch(id, payload));
      },
      pollData(id) {
        return api.posts.pollData(id);
      },
      pollVote(postId, answerId) {
        return this._updatePost(api.posts.pollVote(postId, answerId));
      },
      authorPost(authorId) {
        const promise = api.posts.author(authorId);
        pushEffect([
          promise,
          value => setState(s => ({ ...s, posts: { ...s.posts, value: value }})),
          printerr
        ]);
        return promise;
      }
    },

    kind: {
      available: kinds,
      value: KIND.ALL,
      set(kind) {
        if (this.value === kind) return;
        setState((s) => ({ ...s, kind: { ...this, value: kind } }));
      },
    },

    order: {
      available: orders,
      value: ORDER.RANK.DESC,
      set(order) {
        setState((s) => ({ ...s, order: { ...this, value: order } }));
      },
    },

    tags: {
      available: [],
      value: [],
      add(tag) {
        if (this.value.includes(tag)) return;
        const tags = [...this.value, tag];
        setState((s) => ({ ...s, tags: { ...s.tags, value: tags } }));
      },
      remove(tag) {
        if (!this.value.includes(tag)) return;
        const tags = without(this.value, tag);
        setState((s) => ({ ...s, tags: { ...s.tags, value: tags } }));
      },
      set(tag) {
        const tags = tag instanceof Array ? tag : [tag];
        setState((s) => ({ ...s, tags: { ...s.tags, value: tags } }));
      },
    },

    keywords: {
      value: [],
      add(kw) {
        const keywords = [...this.value, trace(kw)];
        setState((s) => ({
          ...s,
          keywords: { ...s.keywords, value: keywords },
        }));
      },
      remove(kw) {
        if (!this.value.includes(kw)) return;
        const keywords = without(state.keywords.value, kw);
        setState((s) => ({
          ...s,
          keywords: { ...s.keywords, value: keywords },
        }));
      },
    },
  });

  useEffect(
    () =>
      pushEffect([
        api.posts.where(clean(query(state), true)),
        (posts) =>
          setState((s) => ({
            ...s,
            posts: { ...s.posts, value: posts },
          })),
        printerr, // TODO
      ]),
    [
      state.kind.value,
      state.order.value,
      state.tags.value,
      state.keywords.value
    ]
  );

  /* Get the tags on first mount */
  useEffect(
    () =>
      pushEffect([
        api.tags(),
        ({ tags }) =>
          setState((state) => ({
            ...state,
            tags: { ...state.tags, available: tags },
          })),
        printerr, // TODO
      ]),
    []
  );

  return (
    <StreamContext.Provider value={state}>{children}</StreamContext.Provider>
  );
}

export const useStream = () => useContext(StreamContext);
