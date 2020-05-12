import React, { useEffect, useState } from 'react';
import Container from 'react-bootstrap/Container';
import { useParams } from 'react-router-dom';

import { useStream } from 'unanimity/context';
import { Post } from 'unanimity/components';
import { Loading } from 'unanimity/components';


let i = 0;
function Detail(props) {

  const id = Number(useParams().id);
  const stream = useStream();
  const [post, setPost] = useState(null);

  useEffect(() => { stream.posts.of(id) }, []);
  useEffect(() => setPost(stream.focus), [stream.focus])

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
