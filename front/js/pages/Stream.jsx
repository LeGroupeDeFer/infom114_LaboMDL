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
import clsx from 'clsx';
import api from '../lib/api';
import { useRequest } from '../hooks';

function InnerStream({ filter, tags, onClick, show_modal, tag_click }) {
  const query = [
    ['kind', filter],
    ['tags', tags],
  ].reduce((a, [k, v]) => (v ? { ...a, [k]: v } : a), {});
  const posts = usePromise(api.posts.where, [query]);

  return (
    <>
      {posts.map((post) => (
        <Row key={post.id} className="mb-4">
          <Col>
            <Post.Preview
              onClick={onClick}
              post={post}
              show_modal={show_modal}
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
<<<<<<< HEAD
  const [filter, setFilter] = useState('all');
  const [posts, setPosts] = useState([]);
  const [tags, setTags] = useState(null);
  const { login, user, token } = useAuth();
  const caps = token.cap;

=======
  const { user } = useAuth();
  const isLogged = !!user;

  const [filter, setFilter] = useState({ key: 'all', label: 'Actualité' });
>>>>>>> issue_64
  const [postModal, setPostModal] = useState(null);
  const [modalDisplayed, setModalDisplayed] = useState(false);

  const [tags, setTags] = useState([]);
  const [choices, setChoices] = useState([]);
  const [error, tagsData] = useRequest(api.tags, []);

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
            show_modal={showModal}
            tag_click={tagClickHandler}
          />
        </Suspense>

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

      <br />
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
      variant="secondary"
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
      ))}
    </ButtonGroup>
  );
};

Stream.defaultProps = {};

export default Stream;
