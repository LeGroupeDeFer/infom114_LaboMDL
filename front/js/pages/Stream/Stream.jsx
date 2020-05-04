import 'regenerator-runtime';
import React, { Suspense, useState } from 'react';
import {
  Container, Row, Col, Button, Modal, Dropdown, DropdownButton,
  Tooltip, OverlayTrigger
} from 'react-bootstrap';
import { MdSort } from 'react-icons/md';
import { FaSearch, FaTag, FaEdit } from 'react-icons/fa';
import { useAuth } from '../../context/authContext';
import { Link } from 'react-router-dom';
import { Post, SearchBar } from '../../components';
import api from '../../lib/api';


// InnerStream :: Object => Component
function InnerStream({ posts, onClick, showModal, tag_click }) {

  return (
    <>
      {posts.map((post) => (
        <Row key={post.id} className="mb-4">
          <Col>
            <Post.Preview
              onClick={onClick}
              post={post}
              show_modal={showModal}
              onTagClick={tag_click}
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
  const [modalDisplayed, setModalDisplayed] = useState(false);

  function hideModal() {
    setModalDisplayed(false);
  }

  function showModal(postId) {
    setPostModal(null);
    const fetchPost = () => {
      api.posts
        .of(postId)
        .then((post) => {
          setPostModal(post);
        })
        .catch((error) => {});
    };
    fetchPost();
    setModalDisplayed(true);
  }

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

      { /* Header*/ }
      <Row>
        <Col>
          <h1 className="text-dark stream-header">{kind.label}</h1>
          <hr />
        </Col>
      </Row>

      { /* Actions */ }
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

      { /* Posts */ }
      <Suspense fallback={<h3>Chargement des posts...</h3>}>
        <InnerStream
          posts={posts}
          showModal={showModal}
          onClick={tagClickHandler}
        />
      </Suspense>

      { /* Modal */ }
      <Modal
        show={modalDisplayed}
        onHide={() => hideModal()}
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

    </Container>
  );

}

Stream.defaultProps = {};


export default Stream;
