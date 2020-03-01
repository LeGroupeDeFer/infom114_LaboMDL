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
import { MdSort } from 'react-icons/md';





/* Delayed fetching of user posts */
// fetchPosts :: int => Promise<Array<Object>>
const fetchPosts = time => new Promise((resolve, _) => setTimeout(
  () => resolve(Array(3).fill({ id: 0, title: 'Im a post title', type: 'info', text: loremIpsum, username: 'John Coffey', votePoints: 18, createdOn: '2020-02-19T12:59-0500' })
    .concat(Array(2).fill({ id: 0, title: 'Im a post title too', type: 'poll', text: loremIpsum, username: 'John Cena', votePoints: 5, createdOn: '2020-02-29T12:59-0500' }))
  ),
  time
));

// PostList :: Object => Component
const PostList = props => {
  const posts = usePromise(fetchPosts, [fakeLatency]);

  return (
    <>
      {posts.map((post, i) => (
        <div>
          <Row key={i} >
            <Col><Post {...props} {...post} /></Col>
          </Row>
          <br />
        </div>
      ))}

    </>
  );
};

// Stream :: None => Component
const Stream = () => {

  const [filter, setFilter] = useState('all');

  return (
    <Container>
      <br />
      <Row className='justify-content-md-center'>
        <FilterBar onClick={setFilter} currentFilter={filter} />
      </Row>

      <br />
      <Row className='justify-content-end'>
        <SortDropdown />
      </Row>

      <br />
      <Suspense fallback={<h3>Loading posts...</h3>}>
        <PostList currentFilter={filter} />
      </Suspense>
    </Container>
  );
}


// SortDropdown :: None => Component
const SortDropdown = () => {

  const [criteria, setCriteria] = useState('new');
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
