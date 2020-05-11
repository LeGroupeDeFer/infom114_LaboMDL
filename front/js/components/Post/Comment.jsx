import React, { useState } from 'react';
import GoReply from '../../icons/reply.svg';
import Moment from '../Moment';
import { VoteSection } from './Vote';
import { Container, Row, Col, Button, Card, Form } from 'react-bootstrap';
import { Link } from 'react-router-dom';
import { WhenLogged } from '../Auth';
import { useAuth } from 'unanimity/context';
import { VOTE } from '../../lib';

const CommentInteraction = WhenLogged(({ onResponse, onMask, onFlag }) => {
  return (
    <Row className="pl-3">
      <a className="post-footer-btn mr-2" onClick={onResponse}>
        <GoReply size="1em" className="mr-1" />
        <span className="text-muted">Répondre</span>
      </a>
      <a className="post-footer-btn mr-2" onClick={onMask}>
        <span className="text-muted">Masquer</span>
      </a>
      <a className="post-footer-btn mr-2" onClick={onFlag}>
        <span className="text-muted">Signaler</span>
      </a>
    </Row>
  );
});

function Comment({ comment, onComment, onVote }) {
  const isLogged = !!useAuth().user;
  const [reply, setReply] = useState('');

  return (
    <Container className="comment">
      <Row className="comment-first-row">
        <Col>
          <VoteSection
            onVote={onVote}
            score={comment.score || 0}
            isLogged={isLogged}
            vote={comment.userVote || VOTE.NONE}
          />
        </Col>
        <Col>
          <div>
            <span className="text-muted">
              <a
                href={`/profile/${comment.author.id}`}
                className="text-dark mr-1 ml-1"
              >
                {comment.author}
              </a>
              <span className=" mr-1">{comment.score} points</span>
              <span className=" mr-1">·</span>
              <Moment date={comment.creationDate} />
            </span>
          </div>
        </Col>
      </Row>

      <Row className="comment-content">
        <Col>
          <div className="comment-text">{comment.text}</div>
          <CommentInteraction
            onComment={() => console.log('Wanna comment!')}
            onMask={() => console.log('Wanna mask!')}
            onFlag={() => console.log('Wanna flag!')}
          />
          <CommentEditor onComment={(c) => console.log('Commented ' + c)} />
        </Col>
      </Row>
    </Container>
  );
}

function CommentEditor({ onComment }) {
  const isLogged = !!useAuth().user;
  const [comment, setComment] = useState('');

  if (isLogged)
    return (
      <div className="comment-editor">
        <Form.Control
          as="textarea"
          rows="3"
          placeholder="Ajouter un commentaire"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
        />
        <Button
          variant="primary"
          className="float-right my-1"
          onClick={() => (comment ? onComment(comment) : undefined)}
        >
          Commenter
        </Button>
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
