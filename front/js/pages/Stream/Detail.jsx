import React, { useEffect, useState } from 'react';
import Container from 'react-bootstrap/Container';
import { useParams } from 'react-router-dom';

import { useStream } from 'unanimity/context';
import { Post } from 'unanimity/components';
import { Loading } from 'unanimity/components';
import { printerr } from 'unanimity/lib';

function Detail(props) {
  const { id } = useParams();
  const stream = useStream();
  const [post, setPost] = useState(null);
  stream.posts.of(id);

  if (stream.value.some((p) => p.id == id)) {
    // Il l'a fetch
  } else {
    // Montrer un loader
  }

  const LocalPost = post === null ? Loading : Post;
  return (
    <Container className="py-5">
      <br />
      <LocalPost {...props} post={post} className="post-detail" />
    </Container>
  );
}

Detail.defaultProps = {};

export default Detail;
