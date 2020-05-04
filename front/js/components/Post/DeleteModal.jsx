import React, { useState } from 'react';
import { Modal, Button } from 'react-bootstrap';
import api from '../../lib/api';

function DeleteModal({ modalDisplayed, setModalDisplayed, deletePost }) {
  const hideModal = () => setModalDisplayed(false)

  return (
    <Modal
      className="modal-delete"
      show={modalDisplayed}
      onHide={hideModal}
      centered
    >
      <Modal.Header closeButton>
        <Modal.Title>Suprimer le post</Modal.Title>
      </Modal.Header>

      <Modal.Body>
        <br />
        <p>
          Voulez-vous vraiment supprimer votre post ? Cette action est
          irréversible.
        </p>
        <div className="float-right">
          <Button
            variant="light"
            className="mt-1 mr-2"
            onClick={hideModal}
          >
            Annuler
          </Button>
          <Button
            variant="danger"
            className=" mt-1"
            onClick={deletePost}
          >
            Supprimer
          </Button>
        </div>
      </Modal.Body>
    </Modal>
  );
}


export default DeleteModal;
