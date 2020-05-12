import React, { useState } from 'react';
import { useHistory, Link } from 'react-router-dom';
import {
  Container,
  Row,
  Col,
  Badge,
  Dropdown,
  DropdownButton,
  Card,
  OverlayTrigger,
  Tooltip,
  Alert,
} from 'react-bootstrap';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import FacebookSquare from '../../icons/facebook-square.svg';
import { FacebookShareButton } from 'react-share';
import Moment from '../Moment';
import clsx from 'clsx';

import { useAuth } from 'unanimity/context';
import { May } from '../Auth';
import { UpVote, DownVote, Vote, VoteSection } from './Vote';
import {
  kindOf,
  empty,
  last,
  preview,
  previewLength,
  WATCH_EVENT,
} from 'unanimity/lib';
import { Circle, Flexbox } from '../';

import Comment from './Comment';
import Poll from './Poll';
import DeleteModal from './DeleteModal';
import ReportModal from './ReportModal';

/* ------------------------------ Post actions ----------------------------- */

const HidePost = May('post:hide', ({ onClick, hidden }) => {
  if (hidden)
    return (
      <Dropdown.Item as="button" onClick={onClick}>
        <Icon icon="eye" className="mr-2" />
        <span>Rendre visible</span>
      </Dropdown.Item>
    );
  return (
    <Dropdown.Item as="button" onClick={onClick}>
      <Icon icon="eye-slash" className="mr-2" />
      <span>Masquer</span>
    </Dropdown.Item>
  );
});

const LockPost = May('post:lock', ({ onClick, post }) => {
  if (post.locked)
    return (
      <Dropdown.Item as="button" onClick={onClick}>
        <Icon icon="unlock" className="mr-2" />
        <span>Dévérouiller</span>
      </Dropdown.Item>
    );

  return (
    <Dropdown.Item as="button" onClick={onClick}>
      <Icon icon="lock" className="mr-2" />
      <span>Vérouiller</span>
    </Dropdown.Item>
  );
});

const WatchPost = May('post:watch', ({ post }) => {
  const history = useHistory();
  return (
    <Dropdown.Item as="button" onClick={() => history.push(`amend/${post.id}`)}>
      <Icon icon="dove" className="mr-2" />
      <span>Suivre</span>
    </Dropdown.Item>
  );
});

const FlagPost = ({ post, userFlag, onFlag, onFlagCancel }) => {
  if (userFlag) {
    return (
      <Dropdown.Item as="button" onClick={() => onFlagCancel(post)}>
        <Icon icon="flag" className="mr-2" />
        <span>Annuler signalement</span>
      </Dropdown.Item>
    );
  }
  return (
    <Dropdown.Item as="button" onClick={() => onFlag(post)}>
      <Icon icon="flag" className="mr-2" />
      <span>Signaler</span>
    </Dropdown.Item>
  );
};

const DeletePost = ({ owner, onClick }) =>
  owner ? (
    <Dropdown.Item as="button" onClick={() => onClick()}>
      <Icon icon="trash-alt" className="mr-2" />
      <span>Supprimer</span>
    </Dropdown.Item>
  ) : (
    <></>
  );

/* --------------------------------- Utils --------------------------------- */

const WatchSymbol = ({ className }) => (
  <OverlayTrigger
    placement="left"
    overlay={
      <Tooltip id="watched">
        Une attention spéciale est portée à cette publication
      </Tooltip>
    }
  >
    <Circle
      width="2em"
      className={clsx('text-light', 'watch-symbol', className)}
    >
      <Icon icon="dove" />
    </Circle>
  </OverlayTrigger>
);

const LockSymbol = ({ className }) => (
  <OverlayTrigger
    placement="auto"
    overlay={<Tooltip>Cette publication est vérouillée</Tooltip>}
  >
    <Circle
      width="2em"
      className={clsx('text-light', 'lock-symbol', 'ml-2', className)}
    >
      <Icon icon="lock" />
    </Circle>
  </OverlayTrigger>
);

export function WatchStatus({ events, isPreview }) {
  if (empty(events)) return <></>;

  const lastEvent = last(events.sort((a, b) => a.event - b.event));
  const label = WATCH_EVENT[lastEvent.event].doneLabel;
  const icon = WATCH_EVENT[lastEvent.event].icon;
  if (isPreview)
    return (
      <Container className="watch-event-preview">
        <Row>
          <Col xs={8} md={10} className="py-2 px-3">
            <p className="watch-event-content">
              <Icon icon={icon} className="mr-3" />
              {preview(lastEvent.comment, 80)}
            </p>
          </Col>
          <Col xs={4} md={2} className="bg-secondary py-2 px-3 text-center">
            <Moment date={lastEvent.time} relative capitalized />
          </Col>
        </Row>
      </Container>
    );

  // TODO
  return <></>;
}

export function PostContent({ isPreview, post, onComment, onPollVote }) {
  if (isPreview)
    return (
      <div className="post-preview expand-preview">
        <p className="pr-1 expand-preview">
          {preview(post.content, previewLength)}
        </p>
        <Link to={`/detail/${post.id}`}>Lire la suite</Link>
      </div>
    );

  return (
    <div className="post-content">
      <p className="mb-4">{post.content}</p>
      {post.kind === 'poll' && (
        <>
          <Poll
            postId={post.id}
            answers={post.answers}
            userAnswer={post.userAnswer}
            onPollVote={onPollVote}
          />
          <br />
        </>
      )}
    </div>
  );
}

/* --------------------------------- Post ---------------------------------- */

export function Post({
  post,
  onVote,
  onPollVote,
  onFlag,
  onFlagCancel,
  onDelete,
  onHide,
  onTag,
  onLock,
  onWatch,
  onComment,
  isPreview,
  onPreview,
  className,
  ...others
}) {
  const { user } = useAuth();
  const history = useHistory();
  const { path } = useRouteMatch();
  const isLogged = !!user;
  const owner = isLogged && post.author.id === user.id;
  const [editors, setEditors] = useState({});

  function addEditor(commentId) {
    if (editors.hasOwnProperty(commentId)) return;
    let tmp = { ...editors };
    tmp[commentId] = { show: true };
    setEditors(tmp);
  }

  function toggleEditor(commentId) {
    let tmp = { ...editors };
    tmp[commentId].show = !tmp[commentId].show;
    setEditors(tmp);
  }

  const cardProps = isPreview
    ? {
        onClick: (e) =>
          e.target.classList.contains('expand-preview')
            ? history.push(`/detail/${post.id}`)
            : null,
      }
    : {};

  const {
    author,
    kind,
    id,
    createdAt,
    score,
    tags,
    title,
    comments,
    userFlag,
  } = post;

  const cls = clsx(
    'post expand-preview',
    isPreview && 'post-preview',
    post.locked && 'post-locked',
    post.hidden && 'post-hidden',
    post.userFlag && 'post-flagged',
    post.watched && 'post-watched',
    className
  );

  return (
    <Card {...others} {...cardProps} className={cls} id={id}>
      <Card.Header className="post-header">
        <Container className="p-0">
          <Row>
            <Col className="expand-preview" xs={12} sm={10}>
              <h5 className="ml-1 expand-preview">
                <Badge className={`post-${kind} mr-1`}>
                  {kindOf(kind).labelSingular}
                </Badge>
                <span className="mr-1">{title}</span>

                <span className="text-muted post-subtitle">
                  <a href="#" className="post-author text-dark mx-1">
                    {author.firstname} {' ' + author.lastname}
                  </a>
                  <span>-</span>
                  <Moment date={createdAt} className="post-moment" />
                </span>
              </h5>
            </Col>

            <Col className="expand-preview" xs={12} sm={2}>
              <Flexbox reverse align={'center'} className="h-100">
                {post.locked && <LockSymbol className="px-2 ml-3 py-1" />}
                {post.watched && <WatchSymbol className="px-2 ml-3 py-1" />}

                {isLogged && (
                  <DropdownButton
                    drop="left"
                    id={`post-${id}-actions`}
                    title={
                      <div className="px-2 py-1">
                        <Icon icon="ellipsis-h" />
                      </div>
                    }
                    variant="link"
                    className="more btn-link"
                    onClick={() => {}}
                    href="#"
                  >
                    <HidePost
                      hidden={post.hidden}
                      onClick={() => onHide(post)}
                    />

                    <FlagPost
                      post={post}
                      onFlag={onFlag}
                      userFlag={userFlag}
                      onFlagCancel={onFlagCancel}
                    />

                    <DeletePost
                      owner={owner}
                      onClick={() => onDelete(post, path)}
                    />

                    <LockPost onClick={() => onLock(post)} post={post} />

                    <WatchPost
                      post={post}
                      onClick={(event) => onWatch(post, event)}
                    />
                  </DropdownButton>
                )}
              </Flexbox>
            </Col>
          </Row>
        </Container>
      </Card.Header>

      <Card.Body className="post-body p-0 expand-preview">
        <div className="d-flex expand-preview">
          <VoteSection
            onVote={(vote) => onVote(post, vote)}
            score={score}
            isLogged={isLogged}
            isLocked={post.locked}
            vote={post.userVote}
          />

          <div className="px-3 pb-3 pt-2 w-100">
            <div className="mb-1">
              {tags.map((tag) => (
                <a
                  href="#"
                  key={tag}
                  className="mr-2 tag"
                  // onClick={() => onTag(tag)}
                >
                  <Icon icon="tag" className="mr-1" />
                  <span>{tag}</span>
                </a>
              ))}
            </div>

            <PostContent
              isPreview={isPreview}
              post={post}
              onTag={onTag}
              onComment={onComment}
              onPollVote={onPollVote}
              className="expand-preview"
            />

            <Flexbox reverse className="post-footer mt-2">
              <Link
                to={`/detail/${id}`}
                className="post-footer-btn mx-2 d-flex align-items-center"
                href="#"
              >
                <Icon icon="comment-alt" size="1x" className="mr-1" />
                <span className="text-muted">
                  {comments.length}
                  {` commentaire${comments.length > 1 ? 's' : ''}`}
                </span>
              </Link>

              <FacebookShareButton
                url={`https://unanimity.be/detail/${id}`}
                quote={`${title}  - ${author.firstname} ${author.lastname}`}
              >
                <a
                  className="post-footer-btn mx-2 d-flex align-items-center"
                  href="#"
                >
                  <FacebookSquare height="18" className="mr-1 fb-icon" />
                  <span className="text-muted">Partager</span>
                </a>
              </FacebookShareButton>
            </Flexbox>
            {!isPreview && (
              <div className="mt-2">
                <Comment.Editor
                  onComment={(comment) => onComment(post, comment)}
                  type="comment"
                />
                <div className="post-comments">
                  {post.comments.map((comment) => (
                    <Comment
                      comment={comment}
                      onComment={onComment}
                      editors={editors}
                      addEditor={addEditor}
                      toggleEditor={toggleEditor}
                    />
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
        <WatchStatus isPreview={isPreview} events={post.watchEvents} />
      </Card.Body>
    </Card>
  );
}

Object.assign(Post, {
  Comment,
  Vote,
  UpVote,
  DownVote,
  Delete: DeleteModal,
  Report: ReportModal,
});

export default Post;
