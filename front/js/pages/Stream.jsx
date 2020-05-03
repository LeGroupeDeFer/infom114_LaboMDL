import React, { Suspense, useState, useEffect } from 'react';
import {
  Container, Row, Col, Button, Modal, ButtonGroup, Dropdown, DropdownButton,
  Tooltip, OverlayTrigger
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import {faGlobeEurope, faBalanceScale, faInfo, faLightbulb} from '@fortawesome/free-solid-svg-icons';
import { MdSort } from 'react-icons/md';
import usePromise from 'react-promise-suspense';
import { fakeLatency, loremIpsum } from '../lib/dev';
import { components } from 'react-select';
import { FaSearch, FaTag, FaEdit } from 'react-icons/fa';
import { useAuth } from '../context/authContext';
import { Link } from 'react-router-dom';
import { PostPreview, Post, SearchBar } from '../components';
import clsx from 'clsx';


// Stream :: None => Component
const Stream = () => {
  const [filter, setFilter] = useState('all');
  const [posts, setPosts] = useState(usePromise(fetchPosts, [fakeLatency]));
  const [tags, setTags] = useState(null);
  const { login, user } = useAuth();
  const [postModal, setPostModal] = useState(null);
  const [modalDisplayed, setModalDisplayed] = useState(false);
  //const isLogged = user != null ? 1 : 0;
  const isLogged = 1;

  function hideModal() {
    setModalDisplayed(false);
  }

  function showModal(e) {
    const postId = e.currentTarget.closest('.post').getAttribute('id');

    // TODO Fetch the post's data

    const postData = {
      post_id: 1234,
      title: 'Je souhaite devenir champion de la WWE',
      type: 'poll',
      text: loremIpsum,
      username: 'John Cena',
      points: 7,
      createdOn: '2020-03-01T12:59-0500',
      commentNb: 5,
      comments: [
        {
          id: 1,
          text: 'Tu racontes de la merde bro ! ',
          author: 'John Cena',
          created_on: '2020-02-29T12:59-0500',
          points: 12,
          children: [
            {
              id: 34747,
              text: 'Breeehhhhhhh',
              author: 'John Cena',
              created_on: '2020-03-14T12:59-0500',
              points: 0,
              children: [],
            },
            {
              id: 2,
              text: 'tg rdv a l gar du nor. 22h vien seul ',
              author: 'John Casey',
              created_on: '2020-02-29T12:59-0500',
              points: 666,
              children: [
                {
                  id: 3,
                  text: 'Ok.',
                  author: 'John Cena',
                  created_on: '2020-03-14T12:59-0500',
                  points: 0,
                  children: [],
                },
              ],
            },
          ],
        },
        {
          id: 4,
          text: 'Yallah ! ',
          author: 'John Couscous',
          created_on: '2020-02-29T12:59-0500',
          points: -4,
          children: [
            {
              id: 35,
              text: 'Test',
              author: 'John Cena',
              created_on: '2020-03-14T12:59-0500',
              points: 0,
              children: [],
            },
          ],
        },
        {
          id: 5,
          text:
            'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque volutpat vulputate nisl quis pulvinar. Praesent euismod magna metus, quis ultricies nunc sagittis in. Maecenas eleifend pulvinar nunc Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque volutpat vulputate nisl quis pulvinar. Praesent euismod magna metus, quis ultricies nunc sagittis in. Maecenas eleifend pulvinar nunc',
          author: 'John Latin',
          created_on: '2020-02-29T12:59-0500',
          points: -7,
          children: [],
        },
      ],
    };

    setPostModal(postData);

    setModalDisplayed(true);
  }

  function handleChange(selectedOpttion) {
    if (selectedOpttion != null) {
      let tags = selectedOpttion.map(function (x) {
        return {
          value: x.value,
          label: x.label,
        };
      });
      setTags(tags);
    } else {
      setTags(null);
    }
  }

  function tagClickHandler(e) {
    e.stopPropagation();
    let tagValue = e.target.getAttribute('value');
    let tag = {
      value: tagValue,
      label: (
        <span>
          <FaTag /> {tagValue}
        </span>
      ),
    };
    setTags(tag);
    // Scroll to the top
    document.getElementsByTagName('main')[0].scrollTo(0, 0);
  }

  function sortPost(criteria) {
    let sortedPost = [
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
    setPosts(sortedPost);
  }

  return (
    <>
      <SearchBar onChange={handleChange} tags={tags}>
        <FilterBar onClick={setFilter} currentFilter={filter} />
      </SearchBar>
      <Container>

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

        <Suspense fallback={<h3>Loading posts...</h3>}>
          <PostList
            currentFilter={filter}
            posts={posts}
            is_logged={isLogged}
            tag_click={tagClickHandler}
            show_modal={(e) => showModal(e)}
          />
        </Suspense>
        <br />

        <Modal
          show={modalDisplayed}
          onHide={() => hideModal()}
          dialogClassName="modal-80w"
        >
          <Modal.Header closeButton></Modal.Header>
          <Modal.Body>
            <Suspense fallback={<h3>Loading data</h3>}>
              <Post is_logged={isLogged} post_data={postModal} />
            </Suspense>
          </Modal.Body>
        </Modal>
      </Container>

      <br />
    </>
  );
};

/* Delayed fetching of user posts */
// fetchPosts :: int => Promise<Array<Object>>
const fetchPosts = (time) =>
  new Promise((resolve, _) =>
    setTimeout(
      () =>
        resolve([
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
            id: 4,
            title: 'Je suis également un titre',
            type: 'idea',
            text: loremIpsum,
            username: 'John Doe',
            points: 0,
            createdOn: '2020-02-27T12:59-0500',
            commentNb: 4,
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
        ]),
      time
    )
  );

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
