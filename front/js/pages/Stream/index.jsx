import React, { useState } from 'react';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import { useStream } from 'unanimity/context/streamContext';
import { SearchBar } from 'unanimity/components';

import Stream from './Stream';
import { SpecificStream } from './Stream';
import Writer from './Writer';
import Detail from './Detail';

// StreamContent :: None => Component
function StreamContent({ onlySpecificPosts }) {
   const { path } = useRouteMatch();
  const stream = useStream();
  const [state, setState] = useState({
    previewPost: false,
    deletePost: false,
    flagPost: false,
    toast: false,
    toastMsg: '',
    onComment: (post, comment) => stream.posts.comment(post, comment),
    onFlag: (v) => setState((state) => ({ ...state, flagPost: v })),
    onFlagCancel: (post) => {
      stream.posts.flag(post, '', true).then(() =>
        setState((state) => ({
          ...state,
          toast: true,
          toastMsg: 'Votre signalement a été annulé',
        }))
      );
    },
    onHide: (post) => stream.posts.hide(post),
    onPollVote: (postId, answerId) => stream.posts.pollVote(postId, answerId),
    onVote: (post, vote) => stream.posts.vote(post, vote),
    onTag: (tag) => stream.tags.set(tag),
    onWatch: (post) => stream.posts.watch(post),
    onSort: (order) => stream.order.set(order),
    onPreview: (v) => setState((state) => ({ ...state, previewPost: v })),
    onDelete: (v) => setState((state) => ({ ...state, deletePost: v })),
    onToast: (v) => setState({ ...state, toast: v }),
    onDeleteConfirmation: (post) =>
      stream.posts.remove(post).then(() =>
        setState((state) => ({
          ...state,
          deletePost: false,
          toast: false,
        }))
      ),
    onFlagConfirmation: (post, reason) =>
      stream.posts.flag(post, reason, false).then(() =>
        setState((state) => ({
          ...state,
          flagPost: false,
          toast: true,
          toastMsg: 'Votre signalement a été enregistré',
        }))
      ),
  });

  return (
    <>
      { 
      ! onlySpecificPosts ? 
        <>
          <SearchBar variant="kinds" />
          <Switch>
            <Route exact path={path}>
              <Stream {...state} />
            </Route>
            <Route path={`${path}write`}>
              <Writer {...state} />
            </Route>
            <Route path={`${path}detail/:id`}>
              <Detail {...state} />
            </Route>
          </Switch> 
        </>
      :
        <SpecificStream filteredPosts={onlySpecificPosts} {...state} />                 
      }
    </>
  );
}

export default StreamContent;
