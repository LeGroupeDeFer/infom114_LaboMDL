import React from 'react';
import Card from 'react-bootstrap/Card';
import Button from 'react-bootstrap/Button';
import { loremIpsum } from '../utils/dev';

const Post = ({ title, text, ...otherProps }) => (
  <Card {...otherProps}>
    <Card.Body >
      <Card.Title>{title}</Card.Title>
      <Card.Text>{
        text.length > 200 ? `${text.slice(0, 200)}...` : text
      }</Card.Text>
      <Button variant='primary'>Read</Button>
    </Card.Body>
  </Card>
);

Post.defaultProps = {
  title: 'A post',
  text: loremIpsum
};

export default Post;