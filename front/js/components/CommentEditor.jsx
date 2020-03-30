import React from 'react';
import { GoArrowUp } from 'react-icons/go';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Form from 'react-bootstrap/Form';
import clsx from 'clsx';

const CommentEditor = ({
  type,
  is_logged,
  comment_id,
  toggle_comment_editor
}) => {
  var editor = '';

  function cancelClickHandle() {
    toggle_comment_editor(comment_id);
  }

  if (!is_logged && type == 'comment') {
    editor = (
      <Card>
        <Card.Body class="comment-editor comment-editor-guest">
          <p className="text-center">
            <span className="mr-2">
              Il faut être authentifié pour pouvoir laisser un commentaire
            </span>
            <a href="/login" className="button btn btn-primary mr-2">
              Se connecter
            </a>
            <a href="/register" className="button btn btn-secondary">
              S'inscrire
            </a>
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
        />
        <div>
          <Button variant="primary" className="float-right mt-1">
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
        />

        <div className="float-right">
          <Button
            variant="light"
            className="mt-1 mr-1"
            onClick={() => cancelClickHandle()}
          >
            Annuler
          </Button>
          <Button variant="primary" className=" mt-1">
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
