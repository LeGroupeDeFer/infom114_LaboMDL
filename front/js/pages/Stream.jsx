import React, { Suspense, useState, useEffect } from 'react';
import Container from 'react-bootstrap/Container';
import Dropdown from 'react-bootstrap/Dropdown';
import DropdownButton from 'react-bootstrap/DropdownButton';
import Row from 'react-bootstrap/Row';
import { MdSort } from 'react-icons/md';
import usePromise from 'react-promise-suspense';
import Post from '../components/Post';
import { loremIpsum, fakeLatency } from '../utils/dev';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import DropdownButton from 'react-bootstrap/DropdownButton';
import Dropdown from 'react-bootstrap/Dropdown';
import { MdSort, MdSearch } from 'react-icons/md';
import CreatableSelect from 'react-select/creatable';
import { components } from 'react-select';
import { FaSearch, FaTag } from 'react-icons/fa';
import { useAuth } from '../context/authContext';
import Card from 'react-bootstrap/Card';
import clsx from 'clsx';

// Stream :: None => Component
const Stream = () => {
  const [filter, setFilter] = useState('all');
  const [posts, setPosts] = useState(usePromise(fetchPosts, [fakeLatency]));
  const [previewDisplayed, setPreviewDisplayed] = useState(false);
  const [lastPostDiplayed, setLastPostDisplayed] = useState(null);

  const { login, user } = useAuth();

  function togllePreview(e) {
    let postId = e.currentTarget.getAttribute('id');
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
        title: 'Im a post title too',
        type: 'poll',
        text: loremIpsum,
        username: 'John Couscous',
        voteCount: 12,
        createdOn: '2020-02-29T12:59-0500'
      },
      {
        id: 1,
        title: 'Im a post title too',
        type: 'poll',
        text: loremIpsum,
        username: 'John Cena',
        voteCount: 7,
        createdOn: '2020-03-01T12:59-0500'
      },
      {
        id: 3,
        title: 'Im a post title',
        type: 'info',
        text: loremIpsum,
        username: 'John Coffey',
        voteCount: 2,
        createdOn: '2020-02-19T12:59-0500'
      },
      {
        id: 4,
        title: 'Im a post title too',
        type: 'idea',
        text: loremIpsum,
        username: 'John Doe',
        voteCount: 0,
        createdOn: '2020-02-27T12:59-0500'
      }
    ];
    setPosts(sortedPost);
  }

  function tagClickHandler(e) {
    e.stopPropagation();
    let tag = e.target.getAttribute('value');

    // TODO : Add tag to search bar, scroll to the search bar and fetch posts

    window.scrollTo(0, 0);
  }

  return (
    <>
      <Container>
        <br />
        <SearchBar />
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

      <div className={`${clsx(!previewDisplayed && 'container')} `}>
        <Row>
          <Col className={`${clsx(previewDisplayed && 'col-4')} `}>
            <Suspense fallback={<h3>Loading posts...</h3>}>
              <PostList
                currentFilter={filter}
                posts={posts}
                is_logged={user != null ? 1 : 0}
                tag_click={tagClickHandler}
                onClick={e => togllePreview(e)}
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
            title: 'Im a post title too',
            type: 'poll',
            text: loremIpsum,
            username: 'John Cena',
            voteCount: 7,
            createdOn: '2020-03-01T12:59-0500'
          },
          {
            id: 1,
            title: 'Im a post title too',
            type: 'poll',
            text: loremIpsum,
            username: 'John Couscous',
            voteCount: 12,
            createdOn: '2020-02-29T12:59-0500'
          },
          {
            id: 4,
            title: 'Im a post title too',
            type: 'idea',
            text: loremIpsum,
            username: 'John Doe',
            voteCount: 0,
            createdOn: '2020-02-27T12:59-0500'
          },
          {
            id: 3,
            title: 'Im a post title',
            type: 'info',
            text: loremIpsum,
            username: 'John Coffey',
            voteCount: 2,
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
const SearchBar = () => {
  const options = [
    {
      value: 'facInfo',
      label: (
        <span>
          <FaTag /> FacInfo
        </span>
      )
    },
    {
      value: 'facEco',
      label: (
        <span>
          <FaTag /> FacEco
        </span>
      )
    },
    {
      value: 'arsenal',
      label: (
        <span>
          <FaTag /> Arsenal
        </span>
      )
    }
  ];

  const primary = '#A0C55F';

  function handleChange(selectedOpttion) {
    if (selectedOpttion != null) {
      let options = selectedOpttion.map(x => x.value);

      console.log('Option selected:', options);
    } else {
      console.log('No options selected');
    }
  }

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
      placeholder={'Search'}
      styles={customStyles}
      formatCreateLabel={userInput => `Search for" ${userInput}"`}
      onChange={handleChange}
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
  const [title, setTitle] = useState('Sort by');

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
          console.log('top cliqued');
          setTitle('Sort by - Top');
        }}
      >
        Top
      </Dropdown.Item>
      <Dropdown.Item
        as="button"
        onClick={() => {
          props.sortPost('new');
          setTitle('Sort by - New');
        }}
      >
        New
      </Dropdown.Item>
      <Dropdown.Item
        as="button"
        onClick={() => {
          props.sortPost('old');
          setTitle('Sort by - Old');
        }}
      >
        Old
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
        All
      </Button>
      <Button
        variant="secondary"
        className={props.currentFilter == 'decisional' ? 'active' : ''}
        onClick={() => props.onClick('decisional')}
      >
        Decisional
      </Button>
      <Button
        variant="secondary"
        className={props.currentFilter == 'poll' ? 'active' : ''}
        onClick={() => props.onClick('poll')}
      >
        Poll
      </Button>
      <Button
        variant="secondary"
        className={props.currentFilter == 'info' ? 'active' : ''}
        onClick={() => props.onClick('info')}
      >
        Information
      </Button>
      <Button
        variant="secondary"
        className={props.currentFilter == 'idea' ? 'active' : ''}
        onClick={() => props.onClick('idea')}
      >
        Idea submission
      </Button>
    </ButtonGroup>
  );
};

Stream.defaultProps = {};

export default Stream;
