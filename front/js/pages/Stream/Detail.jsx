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
  useEffect(() => {
    let isSubscribed = true;
    stream.posts.of(id)
      .then(post => isSubscribed ? setPost(post) : undefined)
      .catch(printerr);
    return () => isSubscribed = false;
  }, []);

  const LocalPost = post === null ? Loading : Post;

  return (
    <Container className="py-5">
      <LocalPost {...props} post={post} className="post-detail"/>
    </Container>
  );
}

Detail.defaultProps = {};

export default Detail;
