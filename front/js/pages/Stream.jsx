import React, { Suspense, useState, useEffect } from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import usePromise from 'react-promise-suspense';
import Post from '../components/Post';
import { loremIpsum, fakeLatency } from '../utils/dev';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import DropdownButton from 'react-bootstrap/DropdownButton'
import Dropdown from 'react-bootstrap/Dropdown'
import { MdSort, MdSearch } from 'react-icons/md';
import CreatableSelect from 'react-select/creatable';
import { components } from 'react-select';
import { FaSearch, FaTag } from 'react-icons/fa';
import { useAuth } from '../context/authContext';

/* Delayed fetching of user posts */
// fetchPosts :: int => Promise<Array<Object>>
const fetchPosts = time => new Promise((resolve, _) => setTimeout(
  () => resolve([
    { id: 0, title: 'Im a post title too', type: 'poll', text: loremIpsum, username: 'John Cena', voteCount: 7, createdOn: '2020-03-01T12:59-0500' },
    { id: 0, title: 'Im a post title too', type: 'poll', text: loremIpsum, username: 'John Couscous', voteCount: 12, createdOn: '2020-02-29T12:59-0500' },
    { id: 0, title: 'Im a post title too', type: 'idea', text: loremIpsum, username: 'John Doe', voteCount: 0, createdOn: '2020-02-27T12:59-0500' },
    { id: 0, title: 'Im a post title', type: 'info', text: loremIpsum, username: 'John Coffey', voteCount: 2, createdOn: '2020-02-19T12:59-0500' }
  ])
  ,
  time
));

// PostList :: Object => Component
const PostList = props => {

  return (
    <>
      {props.posts.map((post, i) => (
        <Row key={i} className="mb-4">
          <Col><Post {...props} {...post} /></Col>
        </Row>
      ))}
    </>
  );
};


// Stream :: None => Component
const Stream = () => {

  const [filter, setFilter] = useState('all');

  const [posts, setPosts] = useState(usePromise(fetchPosts, [fakeLatency]));

  const { login, user } = useAuth();

  if (!user) {
    // Guest
  }

  function sortPost(criteria) {

    let sortedPost =
      [
        { id: 0, title: 'Im a post title too', type: 'poll', text: loremIpsum, username: 'John Couscous', voteCount: 12, createdOn: '2020-02-29T12:59-0500' },
        { id: 0, title: 'Im a post title too', type: 'poll', text: loremIpsum, username: 'John Cena', voteCount: 7, createdOn: '2020-03-01T12:59-0500' },
        { id: 0, title: 'Im a post title', type: 'info', text: loremIpsum, username: 'John Coffey', voteCount: 2, createdOn: '2020-02-19T12:59-0500' },
        { id: 0, title: 'Im a post title too', type: 'idea', text: loremIpsum, username: 'John Doe', voteCount: 0, createdOn: '2020-02-27T12:59-0500' }
      ];
    setPosts(sortedPost);

  }


  return (
    <Container>
      <br />
      <SearchBar />
      <br />
      <Row className='justify-content-md-center'>
        <FilterBar onClick={setFilter} currentFilter={filter} />
      </Row>

      <br />
      <Row className='justify-content-end'>
        <SortDropdown sortPost={sortPost} />
      </Row>

      <br />
      <Suspense fallback={<h3>Loading posts...</h3>}>
        <PostList currentFilter={filter} posts={posts} isLogged={user != null} />
      </Suspense>
    </Container>
  );
}


// SearchBar :: None => Component
const SearchBar = () => {

  const options = [
    { value: 'facInfo', label: <span><FaTag /> FacInfo</span> },
    { value: 'facEco', label: <span><FaTag /> FacEco</span> },
    { value: 'arsenal', label: <span><FaTag /> Arsenal</span> },
  ];


  const primary = '#A0C55F';

  function handleChange(selectedOpttion) {
    if (selectedOpttion != null) {
      let options = selectedOpttion.map(x => x.value);

      console.log(`Option selected:`, options);
    } else {

      console.log('Noptions selected');
    }


  };

  const customStyles = {
    control: (base, state) => ({
      ...base,
      boxShadow: state.isFocused ? '0 0 0 1px ' + primary : 0,
      borderColor: state.isFocused
        ? primary
        : base.borderColor,
      '&:hover': {
        borderColor: state.isFocused
          ? primary
          : primary,
      }
    }),
    option: (styles, { isFocused }) => ({
      ...styles,
      backgroundColor: isFocused ? primary : null,
    }),
  };

  return (
    <CreatableSelect
      id="search-bar"
      isMulti
      options={options}
      components={{ DropdownIndicator }}
      placeholder={"Search"}
      styles={customStyles}
      formatCreateLabel={userInput => `Search for" ${userInput}"`}
      onChange={handleChange}
    />
  );
}



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
        <span><MdSort size={20} /> {title}</span>
      }
      variant='secondary'
    >
      <Dropdown.Item
        as='button'
        onClick={() => { props.sortPost('top'); console.log("top cliqued"); setTitle('Sort by - Top') }}
      >
        Top
      </Dropdown.Item>
      <Dropdown.Item
        as='button'
        onClick={() => { props.sortPost('new'); setTitle('Sort by - New') }}
      >
        New
      </Dropdown.Item>
      <Dropdown.Item
        as='button'
        onClick={() => { props.sortPost('old'); setTitle('Sort by - Old') }}
      >
        Old
      </Dropdown.Item>
    </DropdownButton>
  );
}

// FilterBar :: Object => Component 
const FilterBar = (props) => {

  return (
    <ButtonGroup id='filter-bar'>
      <Button
        variant='secondary'
        className={props.currentFilter == 'all' ? 'active' : ''}
        onClick={() => props.onClick('all')}
      >
        All
      </Button>
      <Button
        variant='secondary'
        className={props.currentFilter == 'decisional' ? 'active' : ''}
        onClick={() => props.onClick('decisional')}
      >
        Decisional
      </Button>
      <Button
        variant='secondary'
        className={props.currentFilter == 'poll' ? 'active' : ''}
        onClick={() => props.onClick('poll')}
      >
        Poll
      </Button>
      <Button
        variant='secondary'
        className={props.currentFilter == 'info' ? 'active' : ''}
        onClick={() => props.onClick('info')}
      >
        Information
      </Button>
      <Button
        variant='secondary'
        className={props.currentFilter == 'idea' ? 'active' : ''}
        onClick={() => props.onClick('idea')}
      >
        Idea submission
      </Button>
    </ButtonGroup>
  );

}

Stream.defaultProps = {};


export default Stream;
