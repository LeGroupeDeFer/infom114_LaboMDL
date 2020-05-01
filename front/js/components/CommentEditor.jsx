import React, { useState } from 'react';
import { GoArrowUp } from 'react-icons/go';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Form from 'react-bootstrap/Form';
import clsx from 'clsx';
import { Link } from 'react-router-dom';

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

// CommentEditor.defaultProps = {
//   type: 'comment',
//   is_logged: 0
// };

export default CommentEditor;
