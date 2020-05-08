import React, { useState, useEffect } from 'react';
import { GoReply } from 'react-icons/go';
import Moment from 'react-moment';
import { DownVote, UpVote } from './Vote';
import { Container, Row, Col, Button, Card, Form } from 'react-bootstrap';
import { Link } from 'react-router-dom';
import { WhenLogged } from '../Auth';
import { useAuth } from 'unanimity/context';


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
})

function Comment({
  comment,
  onComment,
}) {
  const isLogged = !!useAuth().user;
  const [voted, setVoted] = useState('no');
  const [points, setPoints] = useState(comment.points);

  return (
    <Container className="comment">
      <Row className="comment-first-row">
        <Col className="col-auto vote-col">
          <UpVote
            isLogged={isLogged}
            voted={voted}
            onVote={setVoted}
            points={points}
          />
        </Col>
        <Col>
          <div>
            <span className="text-muted">
              <a href={`/profile/${comment.author.id}`} className="text-dark mr-1 ml-1">{comment.author}</a>
              <span className=" mr-1">{points} points</span>
              <span className=" mr-1">·</span>
              <Moment locale="fr" fromNow>{comment.creationDate}</Moment>
            </span>
          </div>
        </Col>
      </Row>

      <Row className="comment-content">
        <Col className="col-auto vote-col">
          <div id="white-mask">
            <DownVote
              isLogged={isLogged}
              voted={voted}
              set_vote={setVoted}
              points={points}
              set_points={setPoints}
            />
          </div>
        </Col>
        <Col>
          <div className="comment-text">{comment.text}</div>
          <CommentInteraction
            onComment={() => console.log('Wanna comment!')}
            onMask={() => console.log('Wanna mask!')}
            onFlag={() => console.log('Wanna flag!')}
          />
          <CommentEditor onComment={c => console.log('Commented ' + c)} />
        </Col>
      </Row>
    </Container>
  );
}

function CommentEditor ({ onComment }) {
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
          onChange={setComment}
        />
        <Button
          variant="primary"
          className="float-right my-1"
          onClick={() => comment ? onComment(comment) : undefined}
        >
          Commenter
        </Button>
      </div>
    );

  return (
    <Card>
      <Card.Body class="comment-editor comment-editor-guest">
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
