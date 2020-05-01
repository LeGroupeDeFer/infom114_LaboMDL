import React, { Suspense } from 'react';
import Container from 'react-bootstrap/Container';
import usePromise from 'react-promise-suspense';
import { useParams } from 'react-router-dom';
import { useAuth } from '../context/authContext';
import Post from '../components/Post';
import api from '../lib/api';
import Card from 'react-bootstrap/Card';

const PostDetail = () => {
  const { id } = useParams();
  const { user } = useAuth();
  const isLogged = user != null ? 1 : 0;

  const FetchedPost = () => {
    const post = usePromise(api.getPost, [id]);
    return <Post {...post} is_logged={isLogged} />;
  };

  return (
    <Container>
      <br />

      <Suspense fallback={<h3>Chargement ...</h3>}>
        <Card>
          <Card.Body>
            <FetchedPost />
          </Card.Body>
        </Card>
      </Suspense>
    </Container>
  );
};

PostDetail.defaultProps = {};

export default PostDetail;
