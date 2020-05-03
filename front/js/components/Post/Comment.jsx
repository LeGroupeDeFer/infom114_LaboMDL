import React, { useState, useEffect } from 'react';
import { GoReply } from 'react-icons/go';
import Moment from 'react-moment';
import { DownVote, UpVote } from './Vote';
import { Row, Col, Button, Card, Form } from 'react-bootstrap';
import { Link } from 'react-router-dom';

const Comment = ({
 comment,
 is_logged,
 add_comment_editor,
 toggle_comment_editor,
 comment_editors,
 add_reply,
 ancestor_id = comment.id,
}) => {
  const nestedComments = (comment.children || []).map((cmmt) => {
    return (
      <Comment
        ancestor_id={ancestor_id}
        key={cmmt.id}
        comment={cmmt}
        is_logged={is_logged}
        add_comment_editor={add_comment_editor}
        toggle_comment_editor={toggle_comment_editor}
        comment_editors={comment_editors}
        add_reply={add_reply}
      />
    );
  });

  const [voted, setVoted] = useState('no');
  const [pointsState, setPointsState] = useState(comment.points);

  function replyOnClickHandle() {
    if (comment_editors[comment.id] === undefined) {
      add_comment_editor(comment.id, ancestor_id);
    } else {
      toggle_comment_editor(comment.id);
    }
  }
  return (
    <>
      <div className="comment">
        <Row className="comment-first-row">
          <Col className="col-auto vote-col">
            <UpVote
              is_logged={is_logged}
              voted={voted}
              set_vote={setVoted}
              points={pointsState}
              set_points={setPointsState}
            />
          </Col>
          <Col>
            <div>
              <span className="text-muted">
                <a href="#" className="text-dark mr-1 ml-1">
                  {comment.author}
                </a>
                <span className=" mr-1">{pointsState} points</span>
                <span className=" mr-1">·</span>
                <Moment locale="fr" fromNow>
                  {comment.created_on}
                </Moment>
              </span>
            </div>
          </Col>
        </Row>
        <Row className="comment-content">
          <Col className="col-auto vote-col">
            <div id="white-mask">
              <DownVote
                is_logged={is_logged}
                voted={voted}
                set_vote={setVoted}
                points={pointsState}
                set_points={setPointsState}
              />
            </div>
          </Col>
          <Col>
            <div className="comment-text">{comment.text}</div>

            {is_logged && (
              <Row className="pl-3">
                <a
                  className="post-footer-btn mr-2"
                  href="#"
                  onClick={() => replyOnClickHandle()}
                >
                  <GoReply size="1em" className="mr-1" />
                  <span className="text-muted">Répondre</span>
                </a>
                <a className="post-footer-btn mr-2" href="#">
                  <span className="text-muted">Masquer</span>
                </a>
                <a className="post-footer-btn mr-2" href="#">
                  <span className="text-muted">Signaler</span>
                </a>
              </Row>
            )}
            {comment_editors[comment.id] != undefined && (
              <div
                className={
                  !comment_editors[comment.id].isVisible ? 'hidden' : 'test'
                }
              >
                {comment_editors[comment.id].editor}
              </div>
            )}
            {nestedComments}
          </Col>
        </Row>
      </div>
    </>
  );
};

const CommentEditor = ({
 type,
 is_logged,
 comment_id,
 toggle_comment_editor,
 add_comment,
 add_reply,
 ancestor_id,
}) => {
  const [comment, setComment] = useState('');

  let editor = '';

  function cancelClickHandle() {
    toggle_comment_editor(comment_id);
  }

  function handleChange(event) {
    setComment(event.target.value);
  }

  function addComment() {
    add_comment(comment);
    setComment('');
  }

  function addReply() {
    add_reply(comment, comment_id, ancestor_id);
    setComment('');
    cancelClickHandle();
  }

  if (!is_logged && type == 'comment') {
    editor = (
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

  if (is_logged && type == 'comment') {
    editor = (
      <div className="comment-editor">
        <Form.Control
          as="textarea"
          rows="3"
          placeholder="Ajouter un commentaire"
          value={comment}
          onChange={handleChange}
        />
        <div>
          <Button
            variant="primary"
            className="float-right mt-1"
            onClick={() => addComment()}
          >
            Commenter
          </Button>
        </div>
        <br />
      </div>
    );
  }

  if (is_logged && type == 'reply') {
    editor = (
      <div className="comment-editor pt-2">
        <Form.Control
          as="textarea"
          rows="3"
          placeholder="Ajouter une réponse"
          onChange={handleChange}
          value={comment}
        />

        <div className="float-right">
          <Button
            variant="light"
            className="mt-1 mr-1"
            onClick={() => cancelClickHandle()}
          >
            Annuler
          </Button>
          <Button
            variant="primary"
            className=" mt-1"
            onClick={() => addReply()}
          >
            Répondre
          </Button>
        </div>
        <br />
      </div>
    );
  }

  return <>{editor}</>;
};

Comment.Editor = CommentEditor;


export default Comment;
