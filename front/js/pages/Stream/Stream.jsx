import 'regenerator-runtime';
import React, { Suspense, useState } from 'react';
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
import { MdSort } from 'react-icons/md';
import { FaSearch, FaTag, FaEdit } from 'react-icons/fa';
import { useAuth } from '../../context/authContext';
import { Link } from 'react-router-dom';
import { Post, SearchBar } from '../../components';
import api from '../../lib/api';
import DeleteModal from 'unanimity/components/Post/DeleteModal';

// InnerStream :: Object => Component
function InnerStream({
  posts,
  onClick,
  showPreview,
  showDeleteModal,
  showReportModal,
  onTagClick,
}) {
  return (
    <>
      {posts.map((post) => (
        <Row key={post.id} className="mb-4">
          <Col>
            <Post.Preview
              onClick={onClick}
              post={post}
              // showPreviewModal={showPreview}
              showDeleteModal={showDelete}
              showReportModal={showReportModal}
              onTagClick={onTagClick}
            />
          </Col>
        </Row>
      ))}
    </>
  );
}

// SortDropdown :: None => Component
const SortDropdown = (props) => {
  const [criteria, setCriteria] = useState('none');
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
function Stream({ kind, posts, onSort }) {
  const { user, token } = useAuth();
  const isLogged = !!user;

  const [postModal, setPostModal] = useState(null);
  const [previewModalDisplayed, setPreviewModalDisplayed] = useState(false);
  const [deleteModalDisplayed, setDeleteModalDisplayed] = useState(false);
  const [reportModalDisplayed, setReportModalDisplayed] = useState(false);

  const [showNotification, setShowNotification] = useState(false);
  const [notifMsg, setNotifMsg] = useState('');
  const [postInModal, setPostInModal] = useState(null);

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

  const onPostDeleted = () => {
    setPosts(posts.filter((p) => p.id !== postToDelete));
    setNotifMsg('Votre post a bien été supprimé');
    toggleNotification();
    setPostInModal(null);
  };

  const onPostReported = () => {
    setNotifMsg('Votre signalement a été pris en compte');
    toggleNotification();
    setPostInModal(null);
  };

  const showDeleteModal = (id) => {
    setDeleteModalDisplayed(true);
    setPostInModal(id);
  };

  const showReportModal = (id) => {
    setReportModalDisplayed(true);
    setPostInModal(id);
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
          <h1 className="text-dark stream-header">{kind.label}</h1>
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
      <Suspense fallback={<h3>Chargement des posts...</h3>}>
        <InnerStream
          posts={posts}
          onClick={showPreviewModal}
          showDeleteModal={showDeleteModal}
          showReportModal={showReportModal}
          onTagClick={tagClickHandler}
        />
      </Suspense>

      {/* Post modal */}
      <Modal
        id="preview-modal"
        show={previewModalDisplayed}
        onHide={hidePreviewModal}
        dialogClassName="modal-80w"
      >
        <Modal.Header closeButton></Modal.Header>
        <Modal.Body>
          {postModal ? (
            <Post {...postModal} isLogged={isLogged} />
          ) : (
            'Chargement des données...'
          )}
        </Modal.Body>
      </Modal>

      {/* Delete post modal */}
      <DeleteModal
        modalDisplayed={deleteModalDisplayed}
        setModalDisplayed={setDeleteModalDisplayed}
        onPostDeleted={onPostDeleted}
        postToDelete={postToDelete}
      />
      <ReportModal
        modalDisplayed={reportModalDisplayed}
        setModalDisplayed={setReportModalDisplayed}
        PostToReport={postToReport}
        onPostReported={onPostReported}
      />
      <Toast
        className="notification"
        show={showNotification}
        onClose={toggleNotification}
        delay={4000}
        autohide
      >
        <Toast.Header>
          <strong className="mr-auto">{notifMsg}</strong>
        </Toast.Header>
      </Toast>
    </Container>
  );
}

Stream.defaultProps = {};

export default Stream;
