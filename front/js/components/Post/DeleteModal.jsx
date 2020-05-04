import React, { useState } from 'react';
import { Modal, Button } from 'react-bootstrap';
import api from '../../lib/api';

function DeleteModal({ modal_displayed, set_modal_displayed, delete_post }) {
  function hideModal() {
    set_modal_displayed(false);
  }
  const deletePost = () => delete_post();

  return (
    <Modal
      className="modal-delete"
      show={modal_displayed}
      onHide={() => hideModal()}
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
            onClick={() => hideModal()}
          >
            Annuler
          </Button>
          <Button
            variant="danger"
            className=" mt-1"
            onClick={() => deletePost()}
          >
            Supprimer
          </Button>
        </div>
      </Modal.Body>
    </Modal>
  );
}

export default DeleteModal;
