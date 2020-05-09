import React from 'react';
import { Link } from 'react-router-dom';
import {
  Container,
  Row,
  Col,
  Badge,
  Dropdown,
  DropdownButton,
  Card,
} from 'react-bootstrap';
import {
  FaEllipsisH,
  FaEyeSlash,
  FaFacebookSquare,
  FaFlag,
  FaLock,
  FaTag,
  FaTrashAlt,
  FaCrown,
} from 'react-icons/fa';
import { FacebookShareButton } from 'react-share';
import { MdModeComment } from 'react-icons/md';
import Moment from 'react-moment';

import { useAuth } from 'unanimity/context';
import { May } from '../Auth';
import { UpVote, DownVote, Vote, VoteSection } from './Vote';
import { preview, previewLength } from 'unanimity/lib';
import Comment from './Comment';
import DeleteModal from './DeleteModal';
import ReportModal from './ReportModal';
import Poll from './Poll';
import clsx from 'clsx';
import { trace } from '../../lib';

const HidePost = May('post:hide', ({ onClick }) => (
  <Dropdown.Item as="button" onClick={onClick}>
    <FaEyeSlash className="mr-2" />
    <span>Masquer</span>
  </Dropdown.Item>
));

const LockPost = May('post:lock', ({ onClick }) => (
  <Dropdown.Item as="button" onClick={onClick}>
    <FaLock className="mr-2" />
    <span>Vérouiller</span>
  </Dropdown.Item>
));

const PromotePost = May('post:promote', ({ onClick }) => (
  <Dropdown.Item as="button" onClick={onClick}>
    <FaCrown className="mr-2" />
    <span>Promouvoir</span>
  </Dropdown.Item>
));

export function PostContent({ isPreview, post, onComment }) {
  if (isPreview)
    return (
      <div className="post-preview">
        <span className="pr-1">{preview(post.content, previewLength)}</span>
        <Link to={`/detail/${post.id}`}>Lire la suite</Link>
      </div>
    );

  return (
    <div className="post-content">
      <p className="mb-4">{post.content}</p>
      {post.kind === 'poll' && <Poll />}
      <Comment.Editor onComment={(comment) => onComment(post, comment)} />
      <div className="post-comments">
        {post.comments.map((comment) => (
          <Comment comment={comment} onComment={onComment} />
        ))}
      </div>
    </div>
  );
}

export function Post({
  post,
  onVote,
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
  const isLogged = !!user;
  const owner = isLogged && post.author.id === user.id;

  const cardProps = isPreview
    ? {
        onClick: (e) =>
          e.target.classList.contains('expand-preview')
            ? onPreview(post)
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
    className
  );

  return (
    <Card {...others} {...cardProps} className={cls} id={id}>
      <Card.Header className="post-header">
        <Container className="p-0">
          <Row>
            <Col className="expand-preview">
              <h5 className="ml-1 expand-preview">
                <Badge className={`post-${kind} mr-1`}>{kind}</Badge>
                <span className="mr-1">{title}</span>

                <span className="text-muted title-part2">
                  <a href="#" className="text-dark mx-1">
                    <span>{author.firstname}</span>
                    <span className="ml-1">{author.lastname}</span>
                  </a>
                  <span>-</span>
                  <Moment locale="fr" fromNow className="ml-1">
                    {createdAt}
                  </Moment>
                </span>
              </h5>
            </Col>

            <Col className="expand-preview">
              <DropdownButton
                alignRight
                id={`post-${id}-actions`}
                title={
                  <div className="px-2 py-1">
                    <FaEllipsisH />
                  </div>
                }
                variant="link"
                className="float-right more btn-link"
                onClick={() => {}}
                href="#"
              >
                <HidePost onClick={() => onHide(post)} />

                {userFlag != null && !userFlag ? (
                  <Dropdown.Item as="button" onClick={() => onFlag(post)}>
                    <FaFlag className="mr-2" />
                    <span>Signaler</span>
                  </Dropdown.Item>
                ) : (
                  <Dropdown.Item as="button" onClick={() => onFlagCancel(post)}>
                    <FaFlag className="mr-2" />
                    <span>Annuler signalement</span>
                  </Dropdown.Item>
                )}

                {owner && (
                  <Dropdown.Item as="button" onClick={() => onDelete(post)}>
                    <FaTrashAlt className="mr-2" />
                    <span>Supprimer</span>
                  </Dropdown.Item>
                )}

                <LockPost onClick={() => onLock(post)} />

                <PromotePost onClick={() => onPromote(post)} />
              </DropdownButton>
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
            <Card.Text>
              <div className="mb-1">
                {tags.map((tag) => (
                  <a
                    href="#"
                    key={tag}
                    className="mr-2 tag"
                    onClick={() => onTag(tag)}
                  >
                    <FaTag className="mr-1" />
                    <span>{tag}</span>
                  </a>
                ))}
              </div>

              <PostContent
                isPreview={isPreview}
                post={post}
                onTag={onTag}
                onComment={onComment}
              />
            </Card.Text>

            <div className="post-footer mb-2">
              <Link
                to={`/detail/${id}`}
                className="post-footer-btn mr-2"
                href="#"
              >
                <MdModeComment size="1.25em" className="mr-1" />
                <span className="text-muted">
                  <span className="pr-1">{comments.length}</span>
                  {`commentaire${comments.length > 1 ? 's' : ''}`}
                </span>
              </Link>

              <FacebookShareButton
                url={`https://unanimity.be/detail/${id}`}
                quote={`${title}  - ${author.firstname} ${author.lastname}`}
              >
                <a className="post-footer-btn mr-2" href="#">
                  <FaFacebookSquare size="1.25em" className="mr-1" />
                  <span className="text-muted">Partager</span>
                </a>
              </FacebookShareButton>
            </div>
          </div>
        </div>
      </Card.Body>
    </Card>
  );
}

Object.assign(Post, {
  Delete: DeleteModal,
  Comment,
  Vote,
  UpVote,
  DownVote,
  Report: ReportModal,
});

export default Post;
