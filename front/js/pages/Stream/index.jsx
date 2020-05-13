import React, { useState } from 'react';
import { useHistory } from 'react-router-dom';
import { Toast } from 'react-bootstrap';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import { useStream } from 'unanimity/context/streamContext';
import { SearchBar } from 'unanimity/components';
import Post from 'unanimity/components/Post';

import Stream from './Stream';
import { SpecificStream } from './Stream';
import Writer from './Writer';
import Detail from './Detail';
import Amend from './Amend';

// Modals :: Object => Component
function StreamModals({
  toast,
  onToast,
  toastMsg,
  deletePost,
  flagPost,
  deleteComment,
  deleteCommentPostId,
  flagComment,
  flagCommentPostId,
  onDelete,
  onFlag,
  onDeleteComment,
  onFlagComment,
  onDeleteConfirmation,
  onFlagConfirmation,
  onDeleteCommentConfirmation,
  onFlagCommentConfirmation,
}) {
  return (
    <>
      <Post.Delete
        post={deletePost}
        show={!!deletePost}
        onHide={() => onDelete(false)}
        onDelete={onDeleteConfirmation}
        onToast={onToast}
      />

      <Post.DeleteComment
        postId={deleteCommentPostId}
        commentId={deleteComment}
        show={!!deleteComment}
        onHide={() => onDeleteComment(false)}
        onDelete={onDeleteCommentConfirmation}
        onToast={onToast}
      />

      <Post.Report
        post={flagPost}
        show={!!flagPost}
        onHide={() => onFlag(false)}
        onFlag={onFlagConfirmation}
        onToast={onToast}
      />

      <Post.ReportComment
        commentId={flagComment}
        postId={flagCommentPostId}
        show={!!flagComment}
        onHide={() => onFlagComment(false)}
        onFlag={onFlagCommentConfirmation}
        onToast={onToast}
      />
      <Toast
        className="notification"
        show={toast}
        onClose={() => onToast(false)}
        delay={4000}
        autohide
      >
        <Toast.Header>
          <strong className="mr-auto"> {toastMsg}</strong>
        </Toast.Header>
      </Toast>
    </>
  );
}

// StreamContent :: None => Component
function StreamContent({ userId }) {
  const { path } = useRouteMatch();
  const stream = useStream();
  const history = useHistory();
  let pathWhenDelete = path;
  const [state, setState] = useState({
    deletePost: false,
    deleteComment: false,
    deleteCommentPostId: false,
    flagPost: false,
    flagComment: false,
    flagCommentPostId: false,
    toast: false,
    toastMsg: '',
    onDeleteComment: (postId, commentId) =>
      setState((state) => ({
        ...state,
        deleteComment: commentId,
        deleteCommentPostId: postId,
      })),
    onDeleteCommentConfirmation: (postId, commentId) =>
      stream.posts.deleteComment(postId, commentId).then(() => {
        setState((state) => ({
          ...state,
          deleteComment: false,
          toast: true,
          toastMsg: 'Votre commentaire a bien été supprimé',
        }));
      }),
    onCommentVote: (postId, commentId, vote) =>
      stream.posts.commentVote(postId, commentId, vote),
    onComment: (post, comment) => stream.posts.comment(post, comment),
    onReply: (postId, commentId, reply) =>
      stream.posts.reply(postId, commentId, reply),
    onFlag: (v) => setState((state) => ({ ...state, flagPost: v })),
    onFlagComment: (postId, commentId) =>
      setState((state) => ({
        ...state,
        flagComment: commentId,
        flagCommentPostId: postId,
      })),
    onFlagCommentCancel: (id) => {
      stream.posts.flagComment(post, '', true).then(() =>
        setState((state) => ({
          ...state,
          toast: true,
          toastMsg: 'Votre signalement a été annulé',
        }))
      );
    },
    onFlagCancel: (post) => {
      stream.posts.flag(post, '', true).then(() =>
        setState((state) => ({
          ...state,
          toast: true,
          toastMsg: 'Votre signalement a été annulé',
        }))
      );
    },
    //setAuthorPostFilter: (userId) => stream.posts.authorPostFilter(userId),
    onHide: (post) => stream.posts.hide(post),
    onHideComment: (postId, commentId) =>
      stream.posts.hideComment(postId, commentId),
    onPollVote: (postId, answerId) => stream.posts.pollVote(postId, answerId),
    onVote: (post, vote) => stream.posts.vote(post, vote),
    onTag: (tag) => stream.tags.set(tag),
    onWatch: (post, event) => stream.posts.watch(post, event),
    onLock: (post) => stream.posts.lock(post),
    onLockComment: (postId, commentId) =>
      stream.posts.lockComment(postId, commentId),
    onSort: (order) => stream.order.set(order),
    onAuthor: (author) => stream.author.set(author),
    onDelete: (v, p) => {
      pathWhenDelete = p;
      setState((state) => ({ ...state, deletePost: v }));
    },
    onToast: (v) => setState({ ...state, toast: v }),
    onDeleteConfirmation: (post) =>
      stream.posts.remove(post).then(() => {
        setState((state) => ({
          ...state,
          deletePost: false,
          toast: true,
          toastMsg: 'Votre publication a bien été supprimée',
        }));
        if (pathWhenDelete == '/detail/:id') {
          history.push(`/`);
        }
      }),
    onFlagConfirmation: (post, reason) =>
      stream.posts.flag(post, reason, false).then(() => {
        setState((state) => ({
          ...state,
          flagPost: false,
          toast: true,
          toastMsg: 'Votre signalement a été enregistré',
        }));
      }),
    onFlagCommentConfirmation: (postId, commentId, reason) =>
      stream.posts.flagComment(postId, commentId, reason, false).then(() => {
        setState((state) => ({
          ...state,
          flagComment: false,
          toast: true,
          toastMsg: 'Votre signalement a été enregistré',
        }));
      }),
  });

  if (userId)
    return (
      <>
        <StreamModals {...state} />
        <SpecificStream userId={userId} {...state} />
      </>
    );

  return (
    <>
      <SearchBar variant="kinds" pending={stream.pending} />

      <StreamModals {...state} />

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
        <Route path={`${path}amend/:id`}>
          <Amend {...state} />
        </Route>
      </Switch>
    </>
  );
}

export default StreamContent;
