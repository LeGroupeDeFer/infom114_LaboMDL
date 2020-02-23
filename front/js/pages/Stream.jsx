import React, { Suspense } from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import usePromise from 'react-promise-suspense';
import Post from '../components/Post';
import { loremIpsum, fakeLatency } from '../utils/dev';


/* Delayed fetching of user posts */
// fetchPosts :: int => Promise<Array<Object>>
const fetchPosts = time => new Promise((resolve, _) => setTimeout(
  () => resolve(Array(20).fill({ id: 0, title: 'A post', text: loremIpsum })),
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
    <Row>
      <Col><h1>Stream</h1></Col>
    </Row>
    <Suspense fallback={<h3>Loading posts...</h3>}>
      <PostList />
    </Suspense>
  </Container>
);

Stream.defaultProps = {};


export default Stream;
