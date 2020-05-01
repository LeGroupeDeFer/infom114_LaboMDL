import React, { useState } from 'react';
import Form from 'react-bootstrap/Form';
import Container from 'react-bootstrap/Container';
import Card from 'react-bootstrap/Card';
import Select from 'react-select';
import Button from 'react-bootstrap/Button';
import { TiDelete } from 'react-icons/ti';
import { FaPlusSquare, FaTag } from 'react-icons/fa';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import InputGroup from 'react-bootstrap/InputGroup';
import api from '../lib/api';
import { useRequest } from '../hooks';

const CreatePost = () => {
  const [error, data] = useRequest(api.tags, []);


  const tags = (data ? data.tags : []).map((tag) => {
    return {
      id: tag.id,
      value: tag.label,
      label: (
        <span>
          <FaTag /> {tag.label}
        </span>
      )
    };
  });

  return (
    <Container>
      <br />
      <h3>Créer un post</h3>
      <br />
      <CreateForm tags={tags} />
    </Container>
  );
};

function CreateForm(tags) {
  const [category, setCategory] = useState(null);

  const cats = [
    { value: 'idea', label: 'Idée' },
    { value: 'info', label: 'Information' },
    { value: 'poll', label: 'Vote' },
  ];

  // I didn't find another way to add styles to the select
  const primary = '#A0C55F';
  const customStyles = {
    control: (base, state) => ({
      ...base,
      boxShadow: state.isFocused ? '0 0 0 1px ' + primary : 0,
      borderColor: state.isFocused ? primary : base.borderColor,
      '&:hover': {
        borderColor: state.isFocused ? primary : primary,
      },
    }),
    option: (styles, { isFocused, isSelected }) => ({
      ...styles,
      color: isSelected ? '' : '',
      backgroundColor: isFocused ? primary : null,
    }),
  };

  function handleCategoryChange(selectedOpttion) {
    setCategory(selectedOpttion.value);
  }

  return (
    <Card>
      <Card.Body>
        <Form>
          <Row>
            <Col>
              <Select
                options={cats}
                placeholder={'Sélectionner une catégorie'}
                styles={customStyles}
                onChange={handleCategoryChange}
              />
            </Col>
            <Col>
              <Form.Control type="text" placeholder="Titre du post" />
            </Col>
          </Row>

          <br />

          <Select
            options={tags.tags}
            isMulti
            placeholder={'Sélectionner un ou plusieurs tags'}
            styles={customStyles}
          />

          <br />

          <Form.Group>
            <Form.Control as="textarea" rows="5" placeholder="Texte.." />
          </Form.Group>

          {category == 'poll' && <PollSection />}

          <Button variant="primary" className="mt-1 float-right">
            Créer
          </Button>

        </Form>
      </Card.Body>
    </Card>
  );
}

function PollSection() {
  const [pollOptions, setPollOptions] = useState(['', '']);

  function addOption() {
    setPollOptions(pollOptions.concat(['']));
  }
  function removeOption(index) {
    var tmp = [...pollOptions];
    tmp.splice(index, 1);
    setPollOptions(tmp);
  }

  function updateOption(index, val) {
    var tmp = [...pollOptions];
    tmp[index] = val;
    setPollOptions(tmp);
  }

  return (
    <>
      <Form.Group>
        {pollOptions.map((val, index) => (
          <div key={index}>
            {index > 1 ? (
              <>
                <InputGroup>
                  <Form.Control
                    type="text"
                    placeholder={'Option ' + (index + 1)}
                    value={val}
                    onChange={(e) => updateOption(index, e.target.value)}
                  />
                  <InputGroup.Append>
                    <Button
                      variant="outline-danger"
                      onClick={() => removeOption(index)}
                    >
                      <TiDelete size={20} />
                    </Button>
                  </InputGroup.Append>
                </InputGroup>
              </>
            ) : (
                <Form.Control
                  type="text"
                  placeholder={'Option ' + (index + 1)}
                  value={val}
                  onChange={(e) => updateOption(index, e.target.value)}
                />
              )}

            <br />
          </div>
        ))}

        {pollOptions.length < 5 && (
          <a href="#" onClick={addOption}>
            <FaPlusSquare className="mr-1" size={20} />
            Ajouter une option
          </a>
        )}
      </Form.Group>
    </>
  );
}

CreatePost.defaultProps = {};

export default CreatePost;
