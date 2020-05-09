import 'regenerator-runtime';

import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import {
  Container,
  Row,
  Col,
  Button,
  Modal,
  Dropdown,
  DropdownButton,
  Tooltip,
  OverlayTrigger,
  Toast,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { MdSort } from 'react-icons/md';
import { FaEdit } from 'react-icons/fa';
import { useStream } from 'unanimity/context/streamContext';
import { ORDER } from 'unanimity/lib';
import Post from 'unanimity/components/Post';

// InnerStream :: Object => Component
function InnerStream({
  deletePost,
  flagPost,
  previewPost,
  toastMsg,
  toast,
  onDelete,
  onPreview,
  onFlag,
  onFlagCancel,
  onHide,
  onVote,
  onTag,
  onToast,
  onDeleteConfirmation,
  onFlagConfirmation,
  onPromote,
}) {
  const stream = useStream();

  return (
    <div className="stream-content">
      {stream.posts.value.map((post) => (
        <Row key={post.id} className="mb-4">
          <Col>
            <Post
              isPreview
              post={post}
              onDelete={onDelete}
              onFlag={onFlag}
              onFlagCancel={onFlagCancel}
              onHide={onHide}
              onVote={onVote}
              onPreview={onPreview}
              onTag={onTag}
              onPromote={onPromote}
            />
          </Col>
        </Row>
      ))}

      {/* Preview modal */}
      <Modal
        id="preview-modal"
        show={!!previewPost}
        onHide={() => onPreview(false)}
        dialogClassName="modal-80w"
      >
        <Modal.Header closeButton />
        <Modal.Body>
          {previewPost && (
            <Post
              post={previewPost}
              onDelete={onDelete}
              onFlag={onFlag}
              onFlagCancel={onFlagCancel}
              onHide={onHide}
              onVote={(vote) => onVote(post, vote)}
              onPreview={onPreview}
              onTag={onTag}
            />
          )}
        </Modal.Body>
      </Modal>

      {/* Delete post modal */}
      <Post.Delete
        post={deletePost}
        show={!!deletePost}
        onHide={() => onDelete(false)}
        onDelete={onDeleteConfirmation}
        onToast={onToast}
      />

      {/* Report post modal */}
      <Post.Report
        post={flagPost}
        show={!!flagPost}
        onHide={() => onFlag(false)}
        onFlag={onFlagConfirmation}
        onToast={onToast}
      />
      <Toast
        className="notification"
        show={toast}
        onClose={() => onToast(false)}
        delay={4000}
        autohide
      >
        <Toast.Header>
          <strong className="mr-auto"> {toastMsg}</strong>
        </Toast.Header>
      </Toast>
    </div>
  );
}

function SortDropdownItem({ value, label, onSort }) {
  return (
    <Dropdown.Item as="button" onClick={() => onSort(value)}>
      {label}
    </Dropdown.Item>
  );
}

// SortDropdown :: None => Component
function SortDropdown({ onSort }) {
  const [title, setTitle] = useState('Trier par');

  const orders = [
    { label: 'Rang', value: ORDER.RANK.DESC },
    { label: 'Score', value: ORDER.SCORE.DESC },
    { label: 'Récent', value: ORDER.AGE.DESC },
    { label: 'Ancien', value: ORDER.AGE.ASC },
  ];

  const localOnSort = (value, label) => {
    setTitle(`Trier par ${label}`);
    onSort(value);
  };

  return (
    <DropdownButton
      alignRight
      title={
        <span>
          <MdSort size={20} /> {title}
        </span>
      }
      variant="primary"
      id="sort-post"
    >
      {orders.map((order) => (
        <SortDropdownItem
          key={order.value}
          label={order.label}
          value={order.value}
          onSort={() => localOnSort(order.value, order.label)}
        />
      ))}
    </DropdownButton>
  );
}

// Stream :: None => Component
function Stream({ onSort, ...others }) {
  const stream = useStream();

  return (
    <Container className="py-5">
      {/* Header*/}
      <Row>
        <Col>
          <h1 className="text-dark stream-header">
            <Icon icon={stream.kind.value.icon} className="mr-3" />
            <span>{stream.kind.value.label}</span>
          </h1>
          <hr />
        </Col>
      </Row>

      {/* Actions */}
      <Row className="pb-3">
        <Col className="d-flex justify-content-between">
          <Link to="/write" className="shape-circle">
            <OverlayTrigger overlay={<Tooltip>Créer un post</Tooltip>}>
              <Button variant="primary" className="h-100">
                <div className="d-flex text-light">
                  <FaEdit />
                </div>
              </Button>
            </OverlayTrigger>
          </Link>
          <SortDropdown onSort={onSort} />
        </Col>
      </Row>

      {/* Posts */}
      <InnerStream {...others} />
    </Container>
  );
}

export default Stream;
