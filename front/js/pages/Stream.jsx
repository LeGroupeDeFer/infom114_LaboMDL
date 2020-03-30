import React, { Suspense, useState, useEffect } from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import { MdSort } from 'react-icons/md';
import usePromise from 'react-promise-suspense';
import Post from '../components/Post';
import Comment from '../components/Comment';
import { fakeLatency, loremIpsum } from '../lib/dev';
import Button from 'react-bootstrap/Button';

import ButtonGroup from 'react-bootstrap/ButtonGroup';
import DropdownButton from 'react-bootstrap/DropdownButton';
import Dropdown from 'react-bootstrap/Dropdown';
import CreatableSelect from 'react-select/creatable';
import { components } from 'react-select';
import { FaSearch, FaTag, FaEdit } from 'react-icons/fa';
import { useAuth } from '../context/authContext';
import Card from 'react-bootstrap/Card';
import clsx from 'clsx';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import { Link } from 'react-router-dom';

// Stream :: None => Component
const Stream = () => {
  const [filter, setFilter] = useState('all');
  const [posts, setPosts] = useState(usePromise(fetchPosts, [fakeLatency]));
  const [previewDisplayed, setPreviewDisplayed] = useState(false);
  const [lastPostDiplayed, setLastPostDisplayed] = useState(null);
  const [tags, setTags] = useState(null);
  const { login, user } = useAuth();

  function handleChange(selectedOpttion) {
    if (selectedOpttion != null) {
      let tags = selectedOpttion.map(function(x) {
        return {
          value: x.value,
          label: x.label
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
      )
    };

    setTags(tag);

    // Scroll to the top
    document.getElementsByTagName('main')[0].scrollTo(0, 0);

    // TODO fetch posts
  }

  function togglePreview(e) {
    let postId = e.currentTarget.closest('.post').getAttribute('id');
    if (postId == lastPostDiplayed) {
      setPreviewDisplayed(false);
      setLastPostDisplayed(null);
    } else {
      setLastPostDisplayed(postId);
      // TODO : async request to fetch post's data
      setPreviewDisplayed(true);
    }
  }

  if (!user) {
    // Guest
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
        createdOn: '2020-02-29T12:59-0500'
      },
      {
        id: 1,
        title: 'Je suis également un titre',
        type: 'poll',
        text: loremIpsum,
        username: 'John Cena',
        points: 7,
        createdOn: '2020-03-01T12:59-0500'
      },
      {
        id: 3,
        title: 'Im a post title',
        type: 'info',
        text: loremIpsum,
        username: 'John Coffey',
        points: 2,
        createdOn: '2020-02-19T12:59-0500'
      },
      {
        id: 4,
        title: 'Je suis également un titre',
        type: 'idea',
        text: loremIpsum,
        username: 'John Doe',
        points: 0,
        createdOn: '2020-02-27T12:59-0500'
      }
    ];
    setPosts(sortedPost);
  }

  return (
    <>
      <Container>
        <br />

        <Link to="/post/1234">Post detail test</Link>
        <Row>
          <Col xs={11}>
            <SearchBar handle_change={handleChange} tags={tags} />
          </Col>
          <Col xs={1}>
            <OverlayTrigger
              placement="bottom"
              overlay={<Tooltip>Créer un post</Tooltip>}
            >
              <Button variant="primary">
                <FaEdit />
              </Button>
            </OverlayTrigger>
          </Col>
        </Row>
        <br />

        <Row className="justify-content-md-center">
          <FilterBar onClick={setFilter} currentFilter={filter} />
        </Row>

        <br />

        <Row className="justify-content-end">
          <SortDropdown sortPost={sortPost} />
        </Row>
      </Container>
      <br />

      <div className={!previewDisplayed ? 'container' : 'ml-3 mr-2'}>
        <Row>
          <Col className={`${clsx(previewDisplayed && 'col-4')} `}>
            <Suspense fallback={<h3>Loading posts...</h3>}>
              <PostList
                currentFilter={filter}
                posts={posts}
                is_logged={user != null ? 1 : 0}
                tag_click={tagClickHandler}
                preview_click={e => togglePreview(e)}
              />
            </Suspense>
          </Col>

          {previewDisplayed && (
            <Col id="preview-col" className="col-8">
              <Card>
                <Card.Header>Preview (# {lastPostDiplayed})</Card.Header>
                <Card.Body>
                  <Card.Text>{loremIpsum}</Card.Text>
                </Card.Body>
              </Card>
            </Col>
          )}
        </Row>
      </div>
      <Container>
        <Card body>
          <Comments is_logged={user != null ? 1 : 0} />
        </Card>
      </Container>
      <br />
    </>
  );
};

/* Delayed fetching of user posts */
// fetchPosts :: int => Promise<Array<Object>>
const fetchPosts = time =>
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
            createdOn: '2020-03-01T12:59-0500'
          },
          {
            id: 1,
            title: 'Je suis également un titre',
            type: 'poll',
            text: loremIpsum,
            username: 'John Couscous',
            points: 12,
            createdOn: '2020-02-29T12:59-0500'
          },
          {
            id: 4,
            title: 'Je suis également un titre',
            type: 'idea',
            text: loremIpsum,
            username: 'John Doe',
            points: 0,
            createdOn: '2020-02-27T12:59-0500'
          },
          {
            id: 3,
            title: 'Im a post title',
            type: 'info',
            text: loremIpsum,
            username: 'John Coffey',
            points: 2,
            createdOn: '2020-02-19T12:59-0500'
          }
        ]),
      time
    )
  );

// PostList :: Object => Component
const PostList = props => {
  return (
    <>
      {props.posts.map((post, i) => (
        <Row key={i} className="mb-4">
          <Col>
            <Post {...props} {...post} />
          </Col>
        </Row>
      ))}
    </>
  );
};

// SearchBar :: None => Component
const SearchBar = props => {
  const options = [
    {
      value: 'FacInfo',
      label: (
        <span>
          <FaTag /> FacInfo
        </span>
      )
    },
    {
      value: 'FacEco',
      label: (
        <span>
          <FaTag /> FacEco
        </span>
      )
    },
    {
      value: 'Arsenal',
      label: (
        <span>
          <FaTag /> Arsenal
        </span>
      )
    }
  ];

  const primary = '#A0C55F';

  const customStyles = {
    control: (base, state) => ({
      ...base,
      boxShadow: state.isFocused ? '0 0 0 1px ' + primary : 0,
      borderColor: state.isFocused ? primary : base.borderColor,
      '&:hover': {
        borderColor: state.isFocused ? primary : primary
      }
    }),
    option: (styles, { isFocused }) => ({
      ...styles,
      backgroundColor: isFocused ? primary : null
    })
  };

  return (
    <CreatableSelect
      id="search-bar"
      isMulti
      options={options}
      components={{ DropdownIndicator }}
      placeholder={'Rechercher'}
      value={props.tags}
      styles={customStyles}
      formatCreateLabel={userInput => `Rechercher "${userInput}"`}
      onChange={props.handle_change}
    />
  );
};

const DropdownIndicator = props => {
  return (
    <components.DropdownIndicator {...props}>
      <FaSearch size="0.85em" />
    </components.DropdownIndicator>
  );
};

// SortDropdown :: None => Component
const SortDropdown = props => {
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
const FilterBar = props => {
  return (
    <ButtonGroup id="filter-bar">
      <Button
        variant="secondary"
        className={props.currentFilter == 'all' ? 'active' : ''}
        onClick={() => props.onClick('all')}
      >
        Tout
      </Button>
      <Button
        variant="secondary"
        className={props.currentFilter == 'poll' ? 'active' : ''}
        onClick={() => props.onClick('poll')}
      >
        Votes
      </Button>
      <Button
        variant="secondary"
        className={props.currentFilter == 'info' ? 'active' : ''}
        onClick={() => props.onClick('info')}
      >
        Informations
      </Button>
      <Button
        variant="secondary"
        className={props.currentFilter == 'idea' ? 'active' : ''}
        onClick={() => props.onClick('idea')}
      >
        Idées
      </Button>
    </ButtonGroup>
  );
};

const Comments = is_logged => {
  const commentData = {
    post_id: 1234,
    comments: [
      {
        id: 1,
        text: 'Tu racontes de la merde bro ! ',
        author: 'John Cena',
        created_on: '2020-02-29T12:59-0500',
        points: 12,
        children: [
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
                children: []
              }
            ]
          }
        ]
      },
      {
        id: 4,
        text: 'Yallah ! ',
        author: 'John Couscous',
        created_on: '2020-02-29T12:59-0500',
        points: -4,
        children: []
      },
      {
        id: 5,
        text:
          'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque volutpat vulputate nisl quis pulvinar. Praesent euismod magna metus, quis ultricies nunc sagittis in. Maecenas eleifend pulvinar nunc Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque volutpat vulputate nisl quis pulvinar. Praesent euismod magna metus, quis ultricies nunc sagittis in. Maecenas eleifend pulvinar nunc',
        author: 'John Latin',
        created_on: '2020-02-29T12:59-0500',
        points: -7,
        children: []
      }
    ]
  };

  return (
    <>
      {commentData.comments.map(comment => {
        return (
          <Comment key={comment.id} comment={comment} is_logged={is_logged} />
        );
      })}
    </>
  );
};

Stream.defaultProps = {};

export default Stream;
