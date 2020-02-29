import React, { Suspense, useState } from 'react';
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
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faSortAmountDownAlt } from '@fortawesome/free-solid-svg-icons';





/* Delayed fetching of user posts */
// fetchPosts :: int => Promise<Array<Object>>
const fetchPosts = time => new Promise((resolve, _) => setTimeout(
  () => resolve(Array(3).fill({ id: 0, title: 'A post', type: 'info', text: loremIpsum, username: 'John Coffey', vote: 18, createdOn: '2020-02-19T12:59-0500' })
    .concat(Array(2).fill({ id: 0, title: '', type: 'poll', text: loremIpsum, username: 'John Cena', vote: 5, createdOn: '2020-02-29T12:59-0500' }))
  ),
  time
));

// PostList :: Object => Component
const PostList = props => {
  const posts = usePromise(fetchPosts, [fakeLatency]);

  return (
    <>
      {posts.map((post, i) => (
        <Row key={i}>
          <Col><Post {...props} {...post} /></Col>
        </Row>
      ))}
    </>
  );
};

// Stream :: None => Component
const Stream = () => (


  <Container>
    <br />
    <Row className='justify-content-md-center'>
      <FilterBar />
    </Row>

    <br />
    <Row className='justify-content-end'>
      <SortDropdown />
    </Row>

    <br />
    <Suspense fallback={<h3>Loading posts...</h3>}>
      <PostList />
    </Suspense>
  </Container>
);


// SortDropdown :: None => Component
const SortDropdown = () => {

  const [criteria, setCriteria] = useState('new');
  const [title, setTitle] = useState('Sort by');
  
  return (
    <DropdownButton
      title={
        <span><Icon icon={faSortAmountDownAlt}></Icon> {title}</span>
      }
      variant="secondary"
    >
      <Dropdown.Item
        as='button'
        onClick={() => { setCriteria('top'); setTitle('Sort by - Top') }}
      >
        Top
      </Dropdown.Item>
      <Dropdown.Item
        as='button'
        onClick={() => { setCriteria('new'); setTitle('Sort by - New') }}
      >
        New
      </Dropdown.Item>
      <Dropdown.Item
        as='button'
        onClick={() => { setCriteria('old'); setTitle('Sort by - Old') }}
      >
        Old
      </Dropdown.Item>
    </DropdownButton>
  );
}

// FilterBar :: None => Component 
const FilterBar = () => {

  const [filter, setFilter] = useState('all');

  return (
    <ButtonGroup id='filter-bar'>
      <Button
        variant='secondary'
        className={filter == 'all' ? 'active' : ''}
        onClick={() => setFilter('all')}
      >
        All
      </Button>
      <Button
        variant='secondary'
        className={filter == 'decisional' ? 'active' : ''}
        onClick={() => setFilter('decisional')}
      >
        Decisional
      </Button>
      <Button
        variant='secondary'
        className={filter == 'poll' ? 'active' : ''}
        onClick={() => setFilter('poll')}
      >
        Poll
      </Button>
      <Button
        variant='secondary'
        className={filter == 'info' ? 'active' : ''}
        onClick={() => setFilter('info')}
      >
        Information
      </Button>
      <Button
        variant='secondary'
        className={filter == 'idea' ? 'active' : ''}
        onClick={() => setFilter('idea')}
      >
        Idea submission
      </Button>
    </ButtonGroup>
  );

}

Stream.defaultProps = {};


export default Stream;
