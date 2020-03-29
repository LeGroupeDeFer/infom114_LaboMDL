import React from 'react';
import { useParams } from 'react-router-dom';
import Comment from '../components/Comment';

const Post = () => {
  const { id } = useParams();

  const commentData = {
    post_id: 1234,
    comments: [
      {
        id: 1,
        text: 'Example comment here.',
        author: 'user2',
        children: [
          {
            id: 2,
            text: 'Another example comment text.',
            author: 'user3',
            children: [
              {
                id: 3,
                text: 'Another example comment text.',
                author: 'user4',
                children: []
              }
            ]
          }
        ]
      },
      {
        id: 4,
        text: 'Example comment here 2.',
        author: 'user5',
        children: []
      }
    ]
  };

  return (
    <>
      <h1>Post {id}</h1>
      {commentData.comments.map(comment => {
        return <Comment key={comment.id} comment={comment} />;
      })}
    </>
  );
};
