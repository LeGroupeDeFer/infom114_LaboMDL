import React, { Suspense } from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Card from 'react-bootstrap/Card';
import Button from 'react-bootstrap/Button';
import usePromise from 'react-promise-suspense';
import { loremIpsum, fakeLatency } from '../utils';

function Post(props) {
  const { title, text, ...otherProps } = props;
  const preview = text.length > 200 ? `${text.slice(0, 200)}...` : text;

  return (
    <Card {...otherProps}>
      <Card.Body >
        <Card.Title>{title}</Card.Title>
        <Card.Text>{preview}</Card.Text>
        <Button variant='primary'>Read</Button>
      </Card.Body>
    </Card>
  );
}

const fetchPosts = time =>
  new Promise((resolve, _) =>
    setTimeout(() =>
      resolve(Array(20).fill({ title: 'A post', text: loremIpsum })), time)
  );

const Posts = props => {
  const posts = usePromise(fetchPosts, [fakeLatency]);

  return (
    <>
      {posts.map((post, i) =>
        <Row key={i}><Col><Post {...props} {...post} /></Col></Row>)
      }
    </>
  );
}

export default function Stream(props) {
  return (
    <Container>
      <Row>
        <Col><h1>Stream</h1></Col>
      </Row>
      <Suspense fallback={<h3>Loading posts...</h3>}>
        <Posts />
      </Suspense>
    </Container>
  );
}