import React, { useState } from 'react';
import { useHistory } from 'react-router-dom';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import {
  Button,
  ButtonGroup,
  OverlayTrigger,
  Tooltip,
  Modal,
  Toast,
} from 'react-bootstrap';
import Post from 'unanimity/components/Post';
import clsx from 'clsx';

import { useStream } from 'unanimity/context/streamContext';
import { SearchBar } from 'unanimity/components';
import { kinds } from 'unanimity/lib';

import Stream from './Stream';
import Writer from './Writer';
import Detail from './Detail';
import { useAuth } from '../../context';

// FilterBar :: Object => Component
function KindSection() {
  const stream = useStream();

  return (
    <ButtonGroup className="kind-section d-flex justify-content-between">
      {kinds.map((kind) => (
        <OverlayTrigger
          key={kind.key}
          placement="bottom"
          overlay={<Tooltip id={kind.key}>{kind.label}</Tooltip>}
        >
          <Button
            key={kind.key}
            className={clsx(
              'kind-choice',
              stream.kind.value.key === kind.key && 'active'
            )}
            onClick={() => stream.kind.set(kind)}
          >
            <Icon icon={kind.icon} />
          </Button>
        </OverlayTrigger>
      ))}
    </ButtonGroup>
  );
}

// Modals :: Object => Component
function StreamModals({
  deletePost,
  flagPost,
  onDelete,
  toast,
  onToast,
  toastMsg,
  onFlag,
  onDeleteConfirmation,
  onFlagConfirmation,
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

      <Post.Report
        post={flagPost}
        show={!!flagPost}
        onHide={() => onFlag(false)}
        onFlag={onFlagConfirmation}
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
function StreamContent() {
  const { token } = useAuth();
  console.log(token);
  const { path } = useRouteMatch();
  const stream = useStream();
  const history = useHistory();
  let pathWhenDelete = path;
  const [state, setState] = useState({
    previewPost: false,
    deletePost: false,
    flagPost: false,
    toast: false,
    toastMsg: '',
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
    onLock: (post) => stream.posts.lock(post),
    onHide: (post) => stream.posts.hide(post),
    onPollVote: (postId, answerId) => stream.posts.pollVote(postId, answerId),
    onVote: (post, vote) => stream.posts.vote(post, vote),
    onTag: (tag) => stream.tags.set(tag),
    onWatch: (post) => stream.posts.watch(post),
    onSort: (order) => stream.order.set(order),
    onPreview: (v) => setState((state) => ({ ...state, previewPost: v })),
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
  });

  return (
    <>
      <SearchBar>
        <KindSection />
      </SearchBar>

      {/* Delete post modal */}

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
      </Switch>
    </>
  );
}

export default StreamContent;
