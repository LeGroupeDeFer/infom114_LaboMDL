import React, { useEffect, useState } from 'react';
import Container from 'react-bootstrap/Container';
import { useParams } from 'react-router-dom';

import { useStream } from 'unanimity/context';
import { Post } from 'unanimity/components';
import { Loading } from 'unanimity/components';
import {head} from "../../lib";


function Detail(props) {

  const id = Number(useParams().id);
  const stream = useStream();
  const [post, setPost] = useState(null);

  useEffect(() => { stream.posts.of(id, true); }, []);
  useEffect(() => {
    const target = head(stream.posts.value.filter(p => p.id === id));
    setPost(target);
  }, [stream.posts.value])

  if (!post)
    return <Loading />;

  return (
    <Container className="py-5">
      <br />
      <Post {...props} post={post} className="post-detail" />
    </Container>
  );
}

Detail.defaultProps = {};

export default Detail;
