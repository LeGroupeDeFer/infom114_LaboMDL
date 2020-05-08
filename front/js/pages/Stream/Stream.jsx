import 'regenerator-runtime';

import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import {
  Container, Row, Col, Button, Modal, Dropdown, DropdownButton, Tooltip,
  OverlayTrigger, Toast,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { MdSort } from 'react-icons/md';
import { FaTag, FaEdit } from 'react-icons/fa';
import { useStream } from 'unanimity/context/streamContext';
import { api, trace } from 'unanimity/lib';
import Post from 'unanimity/components/Post';


// InnerStream :: Object => Component
function InnerStream() {
  const stream = useStream();

  const [previewModal, setPreviewModal] = useState(false);
  const [deleteModal, setDeleteModal] = useState(false);
  const [toast, setToast] = useState(false);

  const onFlag = post => stream.posts.flag(post);
  const onHide = post => stream.posts.hide(post);
  const onVote = (post, vote) => stream.posts.vote(trace(post), trace(vote));
  const onTag = tag => stream.tags.set(tag);

  const onPreview = post => setPreviewModal(post);
  const onDelete = post => setDeleteModal(post);
  const onDeleteConfirmation =
      post => stream.posts.delete(post).then(() => setToast(true));

  return (
    <>
      {stream.posts.value.map(post => (
        <Row key={post.id} className="mb-4"><Col>
          <Post
            isPreview
            post={post}
            onDelete={onDelete}
            onFlag={onFlag}
            onHide={onHide}
            onVote={vote => onVote(post, vote)}
            onPreview={onPreview}
            onTag={onTag}
          />
        </Col></Row>
      ))}

      {/* Preview modal */}
      <Modal
        id="preview-modal"
        show={!!previewModal}
        onHide={() => setPreviewModal(false)}
        dialogClassName="modal-80w"
      >
        <Modal.Header closeButton />
        <Modal.Body>
          {previewModal && (
            <Post
              post={previewModal}
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
        show={!!deleteModal}
        onHide={() => setDeleteModal(false)}
        onDelete={onDeleteConfirmation}
      />

      <Toast
        className="notification"
        show={toast}
        onClose={() => setToast(false)}
        delay={4000}
        autohide
      >
        <Toast.Header>
          <strong className="mr-auto"> Votre post a bien été supprimé</strong>
        </Toast.Header>
      </Toast>
    </>
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
function Stream({ onSort }) {

  const stream = useStream();

  const [postModal, setPostModal] = useState(null);
  const [previewModalDisplayed, setPreviewModalDisplayed] = useState(false);
  const [deleteModalDisplayed, setDeleteModalDisplayed] = useState(false);
  const [showNotification, setShowNotification] = useState(false);
  const [postToDelete, setPostToDelete] = useState(null);

  /* Preview modal */
  function hidePreviewModal() {
    setPreviewModalDisplayed(false);
  }

  function showPreviewModal(id) {
    setPostModal(null);
    api.posts
      .of(id)
      .then(setPostModal)
      .catch((error) => {});
    setPreviewModalDisplayed(true);
  }

  /* Delete modal */
  const deletePost = () => {
    setDeleteModalDisplayed(false);
    api.posts
      .delete(postToDelete)
      .then(() => {
        // setPosts(posts.filter(p => p.id !== postToDelete));
        toggleNotification();
        setPostToDelete(null);
      })
      .catch((error) => {});
  };

  const showDeleteModal = (id) => {
    setDeleteModalDisplayed(true);
    setPostToDelete(id);
  };

  /* Notification */
  const toggleNotification = () => setShowNotification((n) => !n);

  function tagClickHandler(e) {
    e.stopPropagation();
    let value = e.target.getAttribute('value');
    let tag = {
      value: value,
      label: (
        <span>
          <FaTag /> {value}
        </span>
      ),
    };
    setChoices([tag]);

    // Scroll to the top
    //document.getElementsByTagName('main')[0].scrollTo(0, 0);
  }

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
      <InnerStream
        onClick={showPreviewModal}
        showDelete={showDeleteModal}
        onTagClick={tagClickHandler}
      />
    </Container>
  );
}


export default Stream;
