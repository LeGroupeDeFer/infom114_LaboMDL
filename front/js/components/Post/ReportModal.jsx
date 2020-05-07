import React, { useState } from 'react';
import { Modal, Button } from 'react-bootstrap';
import api from '../../lib/api';

function ReportModal({
  modalDisplayed,
  setModalDisplayed,
  onPostReported,
  postToReport,
}) {
  const [reason, setReason] = useState('');
  const hideModal = () => setModalDisplayed(false);

  function handleChange(event) {
    setReason(event.target.value);
  }

  const reportPost = () => {
    setDeleteModalDisplayed(false);
    api.posts
      .report(postToReport, reason)
      .then(() => {
        onPostReported();
      })
      .catch((error) => {});
  };

  return (
    <Modal
      className="modal-report"
      show={modalDisplayed}
      onHide={hideModal}
      centered
    >
      <Modal.Header closeButton>
        <Modal.Title>Signaler le post</Modal.Title>
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
        <div className="float-right">
          <Button variant="light" className="mt-1 mr-2" onClick={hideModal}>
            Annuler
          </Button>
          <Button variant="danger" className=" mt-1" onClick={reportPost}>
            Signaler
          </Button>
        </div>
      </Modal.Body>
    </Modal>
  );
}

export default ReportModal;
