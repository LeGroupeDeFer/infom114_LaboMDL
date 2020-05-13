import React, { useState } from 'react';
import { useHistory, Link } from 'react-router-dom';
import {
  Container,
  Row,
  Col,
  Badge,
  DropdownButton,
  Card,
  Fade,
  Collapse,
  Button,
} from 'react-bootstrap';
import { useRouteMatch } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import FacebookSquare from '../../icons/facebook-square.svg';
import { FacebookShareButton } from 'react-share';
import Moment from '../Moment';
import clsx from 'clsx';

import { useAuth } from 'unanimity/context';
import { UpVote, DownVote, Vote, VoteSection } from './Vote';
import { kindOf, preview, previewLength } from 'unanimity/lib';
import { Flexbox } from '../';

import Comment from './Comment';
import Poll from './Poll';
import DeleteModal from './DeleteModal';
import ReportModal from './ReportModal';
import DeleteCommentModal from './DeleteCommentModal';
import ReportCommentModal from './ReportCommentModal';
import { empty } from 'unanimity/lib';
import {
  WatchStatus,
  FlagPost,
  LockPost,
  HidePost,
  DeletePost,
  LockSymbol,
  WatchPost,
  WatchSymbol,
} from './Utils';
import { Loading } from 'unanimity/components';

export function PostContent({ isPreview, post, onComment, onPollVote }) {
  const [openCollapse, setOpenCollapse] = useState(false);
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
          {post.pollInfo != undefined ? (
            <Poll
              postId={post.id}
              answers={post.pollInfo.answers}
              userAnswer={post.pollInfo.userAnswer}
              onPollVote={onPollVote}
            />
          ) : (
            <div id="poll-loader">
              <Loading />
            </div>
          )}
        </>
      )}
      {!empty(post.watchEvents) && (
        <>
          <Button
            onClick={() => setOpenCollapse(!openCollapse)}
            aria-controls="status-info"
            aria-expanded={openCollapse}
            className="mb-2"
          >
            Informations de suivi
          </Button>
          <Collapse in={openCollapse}>
            <div id="status-info">
              <WatchStatus events={post.watchEvents} />
            </div>
          </Collapse>
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
  onDeleteComment,
  onReply,
  onCommentVote,
  isPreview,
  onPreview,
  onFlagComment,
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
    if (editors.hasOwnProperty(commentId)) {
      toggleEditor(commentId);
      return;
    }

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
    <Card {...cardProps} className={cls} id={id}>
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
                  <span className="post-author text-dark mx-1">
                    {author.firstname} {' ' + author.lastname}
                  </span>
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

                    <WatchPost post={post} />
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

          <div className="px-3 pb-3 pt-2 w-100 expand-preview">
            <div className="mb-1 expand-preview">
              {tags.map((tag) => (
                <span key={tag} className="mr-2 tag">
                  <Icon icon="tag" className="mr-1" />
                  <span>{tag}</span>
                </span>
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

            <Flexbox reverse className="post-footer mt-2 expand-preview">
              <Link
                to={`/detail/${id}`}
                className="post-footer-btn mx-2 d-flex align-items-center"
              >
                <Icon icon="comment-alt" size="1x" className="mr-1" />
                <span className="text-muted">
                  {post.commentCount}
                  {` commentaire${post.commentCount > 1 ? 's' : ''}`}
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
                  {post.comments.map((comment, index) => (
                    <Comment
                      key={index}
                      comment={comment}
                      onComment={onComment}
                      editors={editors}
                      addEditor={addEditor}
                      toggleEditor={toggleEditor}
                      onReply={onReply}
                      onCommentVote={onCommentVote}
                      onDeleteComment={onDeleteComment}
                      onFlagComment={onFlagComment}
                    />
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      </Card.Body>
      {isPreview && (
        <WatchStatus isPreview={isPreview} events={post.watchEvents} className="expand-preview"/>
      )}
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
  ReportComment: ReportCommentModal,
  DeleteComment: DeleteCommentModal,
});

export default Post;
