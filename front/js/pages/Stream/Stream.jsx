import 'regenerator-runtime';

import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import {
  Container, Row, Col, Button, Modal, Dropdown, DropdownButton, Tooltip,
  OverlayTrigger, Toast,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { MdSort } from 'react-icons/md';
import { FaEdit } from 'react-icons/fa';
import { useStream } from 'unanimity/context/streamContext';
import Post from 'unanimity/components/Post';


// InnerStream :: Object => Component
function InnerStream({
  deletePost, onDelete, previewPost, onPreview, toast, onToast,
  onFlag, onHide, onVote, onTag, onDeleteConfirmation
}) {
  const stream = useStream();

  return (
    <div className="stream-content">
      {stream.posts.value.map(post => (
        <Row key={post.id} className="mb-4"><Col>
          <Post
            isPreview
            post={post}
            onDelete={onDelete}
            onFlag={onFlag}
            onHide={onHide}
            onVote={onVote}
            onPreview={onPreview}
            onTag={onTag}
          />
        </Col></Row>
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
              onHide={onHide}
              onVote={vote => onVote(post, vote)}
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
      />

      <Toast
        className="notification"
        show={toast}
        onClose={() => onToast(false)}
        delay={4000}
        autohide
      >
        <Toast.Header>
          <strong className="mr-auto"> Votre post a bien été supprimé</strong>
        </Toast.Header>
      </Toast>
    </div>
  );

}

// SortDropdown :: None => Component
const SortDropdown = (props) => {
  const [title, setTitle] = useState('Trier par');

  return (
    <DropdownButton
      title={
        <span>
          <MdSort size={20} /> {title}
        </span>
      }
      variant="primary"
      id="sort-post"
    >
      <Dropdown.Item
        as="button"
        onClick={() => {
          props.sortPost('top');
          setTitle('Trier par - Top');
        }}
      >
        Top
      </Dropdown.Item>
      <Dropdown.Item
        as="button"
        onClick={() => {
          props.sortPost('new');
          setTitle('Trier par - Récent');
        }}
      >
        Récent
      </Dropdown.Item>
      <Dropdown.Item
        as="button"
        onClick={() => {
          props.sortPost('old');
          setTitle('Trier par - Ancien');
        }}
      >
        Ancien
      </Dropdown.Item>
    </DropdownButton>
  );
};

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
          <SortDropdown sortPost={onSort} />
        </Col>
      </Row>

      {/* Posts */}
      <InnerStream {...others} />
    </Container>
  );

}


export default Stream;
