import React, { useState } from 'react';
import { Modal, Button } from 'react-bootstrap';

export default function DeleteCommentModal({ post, show, onHide, onDelete }) {
  return (
    <Modal className="modal-delete" show={show} onHide={onHide} centered>
      <Modal.Header closeButton>
        <Modal.Title>Suprimer le commentaire</Modal.Title>
      </Modal.Header>

      <Modal.Body>
        <br />
        <p>
          Voulez-vous vraiment supprimer votre commentaire ? Cette action est
          irréversible.
        </p>
        <div className="float-right">
          <Button variant="light" className="mt-1 mr-2" onClick={onHide}>
            Annuler
          </Button>
          <Button
            variant="danger"
            className=" mt-1"
            onClick={() => onDelete(post)}
          >
            Supprimer
          </Button>
        </div>
      </Modal.Body>
    </Modal>
  );
}
