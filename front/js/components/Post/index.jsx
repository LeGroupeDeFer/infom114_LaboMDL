import React  from 'react';
import { Link } from 'react-router-dom';
import { Container, Row, Col, Badge, Dropdown, DropdownButton } from 'react-bootstrap';
import { FaEllipsisH, FaEyeSlash, FaFacebookSquare, FaFlag, FaLock, FaTag, FaTrashAlt } from 'react-icons/fa';
import { FacebookShareButton } from "react-share";
import { MdModeComment } from "react-icons/md";
import Moment from 'react-moment';

import { May } from '../Auth';

import Comment from './Comment';
import Post from './Post';
import { UpVote, DownVote, Vote } from './Vote';
import DeleteModal from './DeleteModal';
import Poll from "./Poll";
import { preview, previewLength } from 'unanimity/lib';


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

export function PostTags({ tags, onTag }) {
  return (
    <div className="mb-1">
      {tags.map(tag => (
        <a href="#"
           key={tag}
           className="mr-2 tag"
           onClick={() => onTag(tag)}
        >
          <FaTag className="mr-1" />
          <span>{tag}</span>
        </a>
      ))}
    </div>
  );
}

export function PostShare({ id, title, author }) {
  return (
    <FacebookShareButton
      url={`https://unanimity.be/detail/${id}`}
      quote={`${title}  - ${author.firstname} ${author.lastname}`}
    >
      <a className="post-footer-btn mr-2" href="#">
        <FaFacebookSquare size="1.25em" className="mr-1" />
        <span className="text-muted">Partager</span>
      </a>
    </FacebookShareButton>
  );
}

export function PostHeader({
  id, kind, title, author, owner, createdAt, onHide, onFlag, onDelete, onLock
}) {
  return (
    <Container className="p-0">
      <Row>
        <Col>
          <h5 className="ml-1 expand-preview">
            <Badge className={`post-${kind} mr-1`}>{kind}</Badge>
            <span className="mr-1">{title}</span>

            <span className="text-muted title-part2">
              <a href="#" className="text-dark mx-1">
                <span>{author.firstname}</span>
                <span className="ml-1">{author.lastname}</span>
              </a>
              <span>-</span>
              <Moment locale="fr" fromNow className="ml-1">{createdAt}</Moment>
            </span>
          </h5>
        </Col>

        <Col>
          <DropdownButton
            alignRight
            id={`post-${id}-actions`}
            title={<div className="px-2 py-1"><FaEllipsisH /></div>}
            variant="link"
            className="float-right more btn-link"
            onClick={() => {}}
            href="#"
          >
            <HidePost onClick={onHide} />

            <Dropdown.Item as="button" onClick={onFlag}>
              <FaFlag className="mr-2" />
              <span>Signaler</span>
            </Dropdown.Item>

            {owner && (
              <Dropdown.Item as="button" onClick={onDelete}>
                <FaTrashAlt className="mr-2" />
                <span>Supprimer</span>
              </Dropdown.Item>
            )}

            <LockPost onClick={onLock}/>
          </DropdownButton>
        </Col>
      </Row>
    </Container>
  );
}

export function PostFooter({ id, title, comments, author }) {
  return (
    <div className="mb-2">

      <Link to={`/detail/${id}`} className="post-footer-btn mr-2" href="#">
        <MdModeComment size="1.25em" className="mr-1" />
        <span className="text-muted">
          <span className="pr-1">{comments.length}</span>
          {`commentaire${comments.length > 1 ? 's' : ''}`}
        </span>
      </Link>

      <PostShare id={id} title={title} author={author} />

      <a className="post-footer-btn mr-3" href="#">
        <FaEyeSlash size="1.25em" className="mr-1" />
        <span className="text-muted">Masquer</span>
      </a>

      <a className="post-footer-btn" href="#">
        <FaFlag size="1.25em" className="mr-1" />
        <span className="text-muted">Signaler</span>
      </a>
    </div>
  );
}

export function PostContent({ isPreview, id, tags, onTag, kind, content, comments, onComment }) {
  return (
    <>
      <PostTags tags={tags} onTag={onTag} />
      { isPreview ? (
        <>
          <span className="pr-1">{preview(content, previewLength)}</span>
          <Link to={`/detail/${id}`}>Lire la suite</Link>
        </>
      ) : (
        <>
          <p className="mb-4">{content}</p>
          {kind === 'poll' && <Poll />}
          {comments.map(comment => <Comment comment={comment} onComment={onComment} />)}
        </>
      )}
    </>
  );
}


Object.assign(Post, {
  Delete: DeleteModal, Comment, Vote, UpVote, DownVote
});


export default Post;
