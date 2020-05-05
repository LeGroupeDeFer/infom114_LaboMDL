import React, { Suspense, useState } from 'react';
import Container from 'react-bootstrap/Container';
import usePromise from 'react-promise-suspense';
import { useParams } from 'react-router-dom';
import { useAuth } from '../../context/authContext';
import { Post } from '../../components';
import api from '../../lib/api';
import Card from 'react-bootstrap/Card';
import DeleteModal from 'unanimity/components/Post/DeleteModal';

const Detail = ({ post }) => {
  const { id } = useParams();
  const { user } = useAuth();
  const isLogged = !!user;
  const [deleteModalDisplayed, setDeleteModalDisplayed] = useState(false);

  const FetchedPost = () => {
    const post = usePromise(api.posts.of, [id]);
    return (
      <Post
        {...post}
        isLogged={isLogged}
        displayDeleteModal={displayDeleteModal}
      />
    );
  };

  function displayDeleteModal() {
    setDeleteModalDisplayed(true);
  }

  return (
    <Container className="py-5">
      <br />

      <Suspense fallback={<h3>Chargement ...</h3>}>
        <Card className="my-5">
          <Card.Body>
            <FetchedPost />
          </Card.Body>
        </Card>
      </Suspense>
      <DeleteModal modal_displayed={deleteModalDisplayed} />
    </Container>
  );
};

Detail.defaultProps = {};

export default Detail;
