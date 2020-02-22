import React from 'react';
import Card from 'react-bootstrap/Card';
import Button from 'react-bootstrap/Button';
import { loremIpsum } from '../utils/dev';
import { preview } from '../utils';

const Post = ({ title, text, previewLength, ...otherProps }) => (
  <Card {...otherProps}>
    <Card.Body >
      <Card.Title>{title}</Card.Title>
      <Card.Text>{preview(text, previewLength)}</Card.Text>
      <Button variant='primary'>Read</Button>
    </Card.Body>
  </Card>
);

Post.defaultProps = {
  title: 'A post',
  text: loremIpsum,
  previewLength: 200
};

export default Post;