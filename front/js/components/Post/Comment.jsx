import React, { useState } from 'react';
import GoReply from '../../icons/reply.svg';
import Moment from '../Moment';
import { VoteSection } from './Vote';
import { Container, Row, Col, Button, Card, Form } from 'react-bootstrap';
import { Link } from 'react-router-dom';
import { WhenLogged } from '../Auth';
import { useAuth } from 'unanimity/context';
import { VOTE } from '../../lib';
import { May } from '../Auth';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';

const HideComment = May('comment:hide', ({ onMask, hidden }) => {
  if (hidden) {
    return (
      <a href="#" className="post-footer-btn mr-2" onClick={onMask}>
        <span className="text-muted">Rendre visible</span>
      </a>
    );
  }
  return (
    <a href="#" className="post-footer-btn mr-2" onClick={onMask}>
      <span className="text-muted">Masquer</span>
    </a>
  );
});

// const FlagPost = ({ post, userFlag, onFlag, onFlagCancel }) => {
//   if (userFlag) {
//     return (
//       <Dropdown.Item as="button" onClick={() => onFlagCancel(post)}>
//         <Icon icon="flag" className="mr-2" />
//         <span>Annuler signalement</span>
//       </Dropdown.Item>
//     );
//   }
//   return (
//     <Dropdown.Item as="button" onClick={() => onFlag(post)}>
//       <Icon icon="flag" className="mr-2" />
//       <span>Signaler</span>
//     </Dropdown.Item>
//   );
// };

const LockComment = May('comment:hide', ({ onLock }) => {
  return (
    <a href="#" className="post-footer-btn mr-2" onClick={onLock}>
      {/* <Icon icon="lock" className="mr-2" /> */}
      <span className="text-muted">Vérouiller</span>
    </a>
  );
});

const CommentInteraction = WhenLogged(
  ({ onResponse, onFlag, onDelete, onLock, onHide, comment }) => {
    return (
      <Row className="pl-1 pt-1 pb-2">
        <a href="#" className="post-footer-btn mr-2" onClick={onResponse}>
          <GoReply size="1em" className="mr-1" />
          <span className="text-muted">Répondre</span>
        </a>
        <a href="#" className="post-footer-btn mr-2" onClick={onFlag}>
          {/* <Icon icon="flag" className="mr-2" /> */}
          <span className="text-muted">Signaler</span>
        </a>
        <a href="#" className="post-footer-btn mr-2" onClick={onDelete}>
          {/* <Icon icon="trash" className="mr-2" /> */}
          <span className="text-muted">Supprimer</span>
        </a>
        {/* <LockComment onLock={onLock} /> */}
        <HideComment onMask={onHide} hidden={comment.hidden} />
      </Row>
    );
  }
);

function Comment({
  comment,
  onComment,
  onReply,
  onVote,
  onCommentVote,
  editors,
  addEditor,
  toggleEditor,
  onFlagComment,
  onDeleteComment,
  onLockComment,
  onHideComment,
}) {
  const isLogged = !!useAuth().user;
  const [reply, setReply] = useState('');

  const nestedComments = (comment.replies || []).map((cmmt) => {
    return (
      <Comment
        key={cmmt.id}
        ancestorId={comment.id}
        comment={cmmt}
        onReply={onReply}
        addEditor={addEditor}
        toggleEditor={toggleEditor}
        editors={editors}
        onCommentVote={onCommentVote}
        onDeleteComment={onDeleteComment}
        onFlagComment={onFlagComment}
        onLockComment={onLockComment}
        onHideComment={onHideComment}
      />
    );
  });

  return (
    <div className="comment">
      <Row className="comment-row">
        <Col className="col-auto comment-vote comment-content">
          <VoteSection
            className="white-mask"
            onVote={(vote) => onCommentVote(comment.postId, comment.id, vote)}
            score={comment.score || 0}
            isLogged={isLogged}
            vote={comment.userVote || VOTE.NONE}
          />
        </Col>
        <Col className="pt-2">
          <div>
            <span className="text-muted">
              <a
                href={`/profile/${comment.author.id}`}
                className="text-dark mr-1"
              >
                {comment.author.firstname}
                {'  '}
                {comment.author.lastname}
              </a>
              <span className=" mr-1">{comment.score} points</span>
              <span className=" mr-1">·</span>
              <Moment date={comment.createdAt} />
            </span>
          </div>

          <div>
            <div className="comment-text">{comment.content}</div>
            <CommentInteraction
              onLock={() => onLockComment(comment.postId, comment.id)}
              onFlag={() => onFlagComment(comment.postId, comment.id)}
              onResponse={() => addEditor(comment.id)}
              onHide={() => onHideComment(comment.postId, comment.id)}
              onDelete={() => onDeleteComment(comment.postId, comment.id)}
              comment={comment}
            />

            {editors.hasOwnProperty(comment.id) && editors[comment.id].show && (
              <Comment.Editor
                type="reply"
                onReply={(reply) => onReply(comment.postId, comment.id, reply)}
                isVisible={false}
                toggleEditor={() => toggleEditor(comment.id)}
              />
            )}
          </div>
          {nestedComments}
        </Col>
      </Row>
    </div>
  );
}

function CommentEditor({ onReply, onComment, type, toggleEditor }) {
  const isLogged = !!useAuth().user;
  const [comment, setComment] = useState('');

  const onKeyPressed = (e) => {
    if (e.keyCode == 13 && e.shiftKey == false) {
      e.preventDefault();
      addComment();
    }
  };

  const addComment = () => {
    comment ? onComment(comment) : undefined;
    setComment('');
  };

  const addReply = () => {
    console.log(onReply);
    comment ? onReply(comment) : undefined;
    setComment('');
  };

  if (isLogged && type == 'comment')
    return (
      <div className="comment-editor">
        <Form.Control
          as="textarea"
          rows="3"
          placeholder="Ajouter un commentaire"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
          onKeyDown={onKeyPressed}
        />
        <Button
          variant="primary"
          className="float-right my-1"
          onClick={() => addComment()}
        >
          Commenter
        </Button>
      </div>
    );

  if (isLogged && type == 'reply')
    return (
      <div className="comment-editor">
        <Form.Control
          as="textarea"
          rows="3"
          placeholder="Répondre"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
          onKeyDown={onKeyPressed}
        />
        <div className="float-right">
          <Button
            variant="light"
            className="mt-1 mr-1"
            onClick={() => toggleEditor()}
          >
            Annuler
          </Button>
          <Button
            variant="primary"
            className=" mt-1"
            onClick={() => addReply() || toggleEditor()}
          >
            Répondre
          </Button>
        </div>
      </div>
    );

  return (
    <Card>
      <Card.Body className="comment-editor comment-editor-guest">
        <p className="text-center">
          <span className="mr-2">
            Il faut être authentifié pour pouvoir laisser un commentaire
          </span>
          <Link to="/login" className="button btn btn-primary mr-2">
            Se connecter
          </Link>
          <Link to="/register" className="button btn btn-secondary">
            S'inscrire
          </Link>
        </p>
      </Card.Body>
    </Card>
  );
}

Comment.Editor = CommentEditor;

export default Comment;
