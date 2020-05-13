import React, { useState } from 'react';
import { Modal, Button, Form } from 'react-bootstrap';

export default function ReportCommentModal({ post, show, onHide, onFlag }) {
  const [reason, setReason] = useState('');

  function handleChange(event) {
    setReason(event.target.value);
  }

  return (
    <Modal className="modal-report" show={show} onHide={onHide} centered>
      <Modal.Header closeButton>
        <Modal.Title>Signaler le commentaire</Modal.Title>
      </Modal.Header>

      <Modal.Body>
        <br />
        <Form.Control
          as="textarea"
          rows="3"
          placeholder="Dites nous en plus ..."
          onChange={handleChange}
          value={reason}
        />
        <br />
        <div className="float-right">
          <Button variant="light" className="mt-1 mr-2" onClick={onHide}>
            Annuler
          </Button>
          <Button
            variant="primary"
            className=" mt-1"
            onClick={() => {
              onFlag(post, reason);
              setReason('');
            }}
          >
            Signaler
          </Button>
        </div>
      </Modal.Body>
    </Modal>
  );
}
