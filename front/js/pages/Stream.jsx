import 'regenerator-runtime';
import React, { Suspense, useState, useEffect } from 'react';
import {
  Container, Row, Col, Button, Modal, ButtonGroup, Dropdown, DropdownButton,
  Tooltip, OverlayTrigger
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faGlobeEurope, faBalanceScale, faInfo, faLightbulb } from '@fortawesome/free-solid-svg-icons';
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


function InnerStream({ filter, tags, isLogged, onClick }) {

  const posts = usePromise(api.posts, []);

  return (
    <>
      {posts.map(post => (
        <Row key={post.id} className="mb-4">
          <Col>
            <Post.Preview
              isLogged={isLogged}
              onClick={onClick}
              {...post}
            />
          </Col>
        </Row>
      ))}
    </>
  );

}


// Stream :: None => Component
const Stream = () => {
  const { user } = useAuth();
  const isLogged = !!user;

  const [filter, setFilter] = useState('all');
  const [posts, setPosts] = useState([]);
  const [postModal, setPostModal] = useState(null);
  const [modalDisplayed, setModalDisplayed] = useState(false);

  const [tags, setTags] = useState([]);
  const [choices, setChoices] = useState([]);
  const [error, tagsData] = useRequest(api.tags, []);

  useEffect(() => setTags((tagsData ? tagsData.tags : []).map(
    tag => ({ id: tag.id, label: tag.label })
  )), [tagsData]);


  function hideModal() {
    setModalDisplayed(false);
  }

  function showModal(postId) {
    setPostModal(null);
    const fetchPost = () => {
      api
        .getPost(postId)
        .then((post) => {
          setPostModal(post);
        })
        .catch((error) => {});
    };

    fetchPost();
    setModalDisplayed(true);
  }

  function handleChange(selectedOptions) {
    setChoices(selectedOptions != null
      ? selectedOptions.map(({ label, value }) => ({ label, value }))
      : []
    );
  }

  function tagClickHandler(e) {
    e.stopPropagation();
    let value = e.target.getAttribute('value');
    let tag = { value, label: value };
    setChoices([tag]);
    // Scroll to the top
    document.getElementsByTagName('main')[0].scrollTo(0, 0);
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
      <Container className="my-5">

        <br />
        <Row>
          <Col xs={2} sm={1}>
            <Link to="/submit">
              <OverlayTrigger
                placement="bottom"
                overlay={<Tooltip>Créer un post</Tooltip>}
              >
                <Button variant="primary">
                  <FaEdit />
                </Button>
              </OverlayTrigger>
            </Link>
          </Col>
        </Row>

        <br />

        <Row className="justify-content-end">
          <SortDropdown sortPost={sortPost} />
        </Row>

        <br />

        <Suspense fallback={<h3>Chargement des posts...</h3>}>
          <InnerStream />
        </Suspense>
        <br />

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

// PostList :: Object => Component
const PostList = (props) => {
  return (
    <>
      {props.posts.map((post, i) => (
        <Row key={i} className="mb-4">
          <Col>
            <PostPreview {...props} {...post} />
          </Col>
        </Row>
      ))}
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
    { label: 'Tout', key: 'all', icon: faGlobeEurope },
    { label: 'Sondage', key: 'poll', icon: faBalanceScale },
    { label: 'Info', key: 'info', icon: faInfo },
    { label: 'Idée', key: 'idea', icon: faLightbulb }
  ];

  return (
    <ButtonGroup className="filter-bar d-flex justify-content-between">
      {options.map(({ key, icon }) => (
        <Button
          key={key}
          className={clsx('filter-choice', currentFilter === key && 'active')}
          onClick={() => onClick(key)}
        >
          <Icon icon={icon} />
        </Button>
      ))}
    </ButtonGroup>
  );
};

Stream.defaultProps = {};


export default Stream;
