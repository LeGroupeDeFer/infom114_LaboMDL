import React from 'react';
import { useParams } from 'react-router-dom';
import Comment from '../components/Comment';

const Post = () => {
  const { id } = useParams();

  return (
    <>
      <h1>Post {id}</h1>
    </>
  );
};

Post.defaultProps = {};

export default Post;
