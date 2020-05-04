import 'regenerator-runtime';
import React, { Suspense, useState, useEffect } from 'react';
import {
  Container,
  Row,
  Col,
  Button,
  Modal,
  ButtonGroup,
  Dropdown,
  DropdownButton,
  Tooltip,
  OverlayTrigger,
  Toast,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import {
  faGlobeEurope,
  faBalanceScale,
  faInfo,
  faLightbulb,
} from '@fortawesome/free-solid-svg-icons';
import { MdSort } from 'react-icons/md';
import usePromise from 'react-promise-suspense';
import { loremIpsum } from '../lib/dev';
import { components } from 'react-select';
import { FaSearch, FaTag, FaEdit } from 'react-icons/fa';
import { useAuth } from '../context/authContext';
import { Link } from 'react-router-dom';
import { Post, SearchBar } from '../components';
import DeleteModal from '../components/Post/DeleteModal';
import clsx from 'clsx';
import api from '../lib/api';
import { useRequest } from '../hooks';

function InnerStream({
  filter,
  tags,
  onClick,
  show_delete_modal,
  show_preview_modal,
  tag_click,
  set_posts,
  posts,
}) {
  const query = [
    ['kind', filter],
    ['tags', tags.filter((t) => t.value != t.label).map((t) => t.value)],
    ['search', tags.filter((t) => t.value === t.label).map((t) => t.value)],
  ].reduce((a, [k, v]) => (v ? { ...a, [k]: v } : a), {});
  set_posts(usePromise(api.posts.where, [query]));

  return (
    <>
      {posts.map((post) => (
        <Row key={post.id} className="mb-4">
          <Col>
            <Post.Preview
              onClick={onClick}
              post={post}
              show_delete_modal={show_delete_modal}
              show_preview_modal={show_preview_modal}
              onTagClick={tag_click}
            />
          </Col>
        </Row>
      ))}
    </>
  );
}

// Stream :: None => Component
const Stream = () => {
  const { user, token } = useAuth();
  const isLogged = !!user;
  const [posts, setPosts] = useState([]);
  const caps = token && token.cap;

  const [filter, setFilter] = useState({ key: 'all', label: 'Actualité' });
  const [postModal, setPostModal] = useState(null);
  const [previewModalDisplayed, setPreviewModalDisplayed] = useState(false);
  const [deleteModalDisplayed, setDeleteModalDisplayed] = useState(false);
  const [postToDelete, setPostToDelete] = useState(-1);
  const [tags, setTags] = useState([]);
  const [choices, setChoices] = useState([]);
  const [error, tagsData] = useRequest(api.tags, []);
  const [showNotif, setShowNotif] = useState(false);

  const deletePost = () => {
    setDeleteModalDisplayed(false);
    const del = () => {
      api.posts
        .delete(postToDelete)
        .then(() => {
          let index = posts.findIndex((p) => p.id == postToDelete);
          let tmp = [...posts];
          tmp.splice(index, 1);
          setPosts(tmp);
          toggleNotif();
          setPostToDelete(-1);
        })
        .catch((error) => {});
    };
    del();
  };

  const toggleNotif = () => setShowNotif(!showNotif);

  useEffect(
    () =>
      setTags(
        (tagsData ? tagsData.tags : []).map((tag) => ({
          id: tag.id,
          label: tag.label,
        }))
      ),
    [tagsData]
  );

  const showDeleteModal = (id) => {
    setDeleteModalDisplayed(true);
    setPostToDelete(id);
  };

  const hidePreviewModal = () => {
    setPreviewModalDisplayed(false);
  };

  function showPreviewModal(postId) {
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
    setPreviewModalDisplayed(true);
  }
  function handleChange(selectedOptions) {
    setChoices(
      selectedOptions != null
        ? selectedOptions.map(({ label, value }) => ({ label, value }))
        : []
    );
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

  function sortPost(criteria) {
    return [
      {
        id: 1,
        title: 'Je suis également un titre',
        type: 'poll',
        text: loremIpsum,
        username: 'John Couscous',
        points: 12,
        createdOn: '2020-02-29T12:59-0500',
        commentNb: 1,
      },
      {
        id: 2,
        title: 'Je suis également un titre',
        type: 'poll',
        text: loremIpsum,
        username: 'John Cena',
        points: 7,
        createdOn: '2020-03-01T12:59-0500',
        commentNb: 2,
      },
      {
        id: 3,
        title: 'Im a post title',
        type: 'info',
        text: loremIpsum,
        username: 'John Coffey',
        points: 2,
        createdOn: '2020-02-19T12:59-0500',
        commentNb: 0,
      },
      {
        id: 4,
        title: 'Je suis également un titre',
        type: 'idea',
        text: loremIpsum,
        username: 'John Doe',
        points: 0,
        createdOn: '2020-02-27T12:59-0500',
        commentNb: 4,
      },
    ];
  }

  return (
    <>
      <SearchBar onChange={handleChange} tags={tags} choices={choices}>
        <FilterBar onClick={setFilter} currentFilter={filter} />
      </SearchBar>
      <Container className="py-5">
        <Row>
          <Col>
            <h1 className="text-dark stream-header">{filter.label}</h1>
            <hr />
          </Col>
        </Row>

        <Row className="pb-3">
          <Col className="d-flex justify-content-between">
            <Link to="/submit" className="shape-circle">
              <OverlayTrigger overlay={<Tooltip>Créer un post</Tooltip>}>
                <Button variant="primary" className="h-100">
                  <div className="d-flex text-light">
                    <FaEdit />
                  </div>
                </Button>
              </OverlayTrigger>
            </Link>
            <SortDropdown sortPost={sortPost} />
          </Col>
        </Row>

        <Suspense fallback={<h3>Chargement des posts...</h3>}>
          <InnerStream
            filter={filter.key}
            tags={choices}
            show_preview_modal={showPreviewModal}
            tag_click={tagClickHandler}
            show_delete_modal={showDeleteModal}
            post_to_delete={postToDelete}
            set_posts={setPosts}
            posts={posts}
          />
        </Suspense>
        <br />
        <Modal
          className="modal-post"
          show={previewModalDisplayed}
          onHide={() => hidePreviewModal()}
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
        <DeleteModal
          modal_displayed={deleteModalDisplayed}
          set_modal_displayed={setDeleteModalDisplayed}
          delete_post={deletePost}
        />
        <Toast
          className="notification"
          show={showNotif}
          onClose={toggleNotif}
          delay={4000}
          autohide
        >
          <Toast.Header>
            <strong className="mr-auto"> Le post a bien été supprimé</strong>
          </Toast.Header>
        </Toast>
      </Container>
    </>
  );
};

const DropdownIndicator = (props) => {
  return (
    <components.DropdownIndicator {...props}>
      <FaSearch size="0.85em" />
    </components.DropdownIndicator>
  );
};

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

// FilterBar :: Object => Component
const FilterBar = ({ currentFilter, onClick }) => {
  const options = [
    { label: 'Actualité', key: 'all', icon: faGlobeEurope },
    { label: 'Sondages', key: 'poll', icon: faBalanceScale },
    { label: 'Infos', key: 'info', icon: faInfo },
    { label: 'Idées', key: 'idea', icon: faLightbulb },
  ];

  return (
    <ButtonGroup className="filter-bar d-flex justify-content-between">
      {options.map(({ key, icon, label }) => (
        <OverlayTrigger placement="down" overlay={<Tooltip>{label}</Tooltip>}>
          <Button
            key={key}
            className={clsx(
              'filter-choice',
              currentFilter.key === key && 'active'
            )}
            onClick={() => onClick({ key, icon, label })}
          >
            <Icon icon={icon} />
          </Button>
        </OverlayTrigger>
      ))}
    </ButtonGroup>
  );
};

Stream.defaultProps = {};

export default Stream;
