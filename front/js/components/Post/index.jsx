import React from 'react';
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
} from 'react-bootstrap';

import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import FacebookSquare from '../../icons/facebook-square.svg';
import { FacebookShareButton } from 'react-share';
import Moment from '../Moment';
import clsx from 'clsx';

import { useAuth } from 'unanimity/context';
import { May } from '../Auth';
import { UpVote, DownVote, Vote, VoteSection } from './Vote';
import { preview, previewLength } from 'unanimity/lib';
import { Circle, Flexbox } from '../';

import Comment from './Comment';
import Poll from './Poll';
import DeleteModal from './DeleteModal';
import ReportModal from './ReportModal';

/* ------------------------------ Post actions ----------------------------- */

const HidePost = May('post:hide', ({ onClick }) => (
  <Dropdown.Item as="button" onClick={onClick}>
    <Icon icon="eye-slash" className="mr-2" />
    <span>Masquer</span>
  </Dropdown.Item>
));

const LockPost = May('post:lock', ({ onClick }) => (
  <Dropdown.Item as="button" onClick={onClick}>
    <Icon icon="lock" className="mr-2" />
    <span>Vérouiller</span>
  </Dropdown.Item>
));

const WatchPost = May('post:watch', ({ onClick }) => (
  <Dropdown.Item as="button" onClick={onClick}>
    <Icon icon="dove" className="mr-2" />
    <span>Promouvoir</span>
  </Dropdown.Item>
));

const FlagPost = ({ post, userFlag, onFlag, onFlagCancel }) => {
  if (userFlag != null && !userFlag)
    return (
      <Dropdown.Item as="button" onClick={() => onFlag(post)}>
        <Icon icon="flag" className="mr-2" />
        <span>Signaler</span>
      </Dropdown.Item>
    );
  return (
    <Dropdown.Item as="button" onClick={() => onFlagCancel(post)}>
      <Icon icon="flag" className="mr-2" />
      <span>Annuler signalement</span>
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
      className={clsx('bg-secondary', 'text-light', 'watch-symbol', className)}
    >
      <Icon icon="dove" />
    </Circle>
  </OverlayTrigger>
);

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
      <Comment.Editor onComment={(comment) => onComment(post, comment)} />
      <div className="post-comments">
        {post.comments.map((comment) => (
          <Comment comment={comment} onComment={onComment} />
        ))}
      </div>
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
  onPromote,
  onComment,
  isPreview,
  onPreview,
  className,
  ...others
}) {
  const { user } = useAuth();
  const history = useHistory();
  const isLogged = !!user;
  const owner = isLogged && post.author.id === user.id;

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
    userVote,
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
            <Col className="expand-preview" sm={10}>
              <h5 className="ml-1 expand-preview">
                <Badge className={`post-${kind} mr-1`}>{kind}</Badge>
                <span className="mr-1">{title}</span>

                <span className="text-muted title-part2">
                  <a href="#" className="text-dark mx-1">
                    {author.firstname} {' ' + author.lastname}
                  </a>
                  <span>-</span>
                  <Moment date={createdAt} />
                </span>
              </h5>
            </Col>

            <Col className="expand-preview">
              <Flexbox reverse align={'center'} className="h-100">
                {post.watched && <WatchSymbol className="px-2 ml-2 py-1" />}

                <DropdownButton
                  alignRight
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
                  <HidePost onClick={() => onHide(post)} />

                  <FlagPost
                    post={post}
                    onFlag={onFlag}
                    userFlag={userFlag}
                    onFlagCancel={onFlagCancel}
                  />

                  <DeletePost owner={owner} onClick={() => onDelete(post)} />

                  <LockPost onClick={() => onLock(post)} />

                  <WatchPost onClick={() => onWatch(post)} />
                </DropdownButton>
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
            vote={userVote}
          />

          <div className="px-3 pb-3 pt-2 w-100">
            <div className="mb-1">
              {tags.map((tag) => (
                <a
                  href="#"
                  key={tag}
                  className="mr-2 tag"
                  onClick={() => onTag(tag)}
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
                <Icon icon={faCommentAlt} size="1x" className="mr-1" />
                <span className="text-muted">
                  {comments.length}
                  {` commentaire${comments.length > 1 ? 's' : ''}`}
                </span>
              </Link>

              <FacebookShareButton
                url={`https://unanimity.be/detail/${id}`}
                quote={`${title}  - ${author.firstname} ${author.lastname}`}
              >
                <a className="post-footer-btn mr-2" href="#">
                  <FacebookSquare height="18" className="mr-1 fb-icon" />
                  <span className="text-muted">Partager</span>
                </a>
              </FacebookShareButton>
            </Flexbox>
          </div>
        </div>
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
