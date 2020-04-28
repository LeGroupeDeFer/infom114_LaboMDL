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

// export default function About(props) {
//   return (
//     <h1>About</h1>
//   );
// }

const handleSubmit = () => console.log('submit');

const About = () => {
  return (
    <Container>
      <br />
      <h3>Créer un post</h3>
      <br />
      <CreateForm />
    </Container>
  );
};

function CreateForm() {
  const [category, setCategory] = useState(null);

  const cats = [
    { value: 'idea', label: 'Idée' },
    { value: 'info', label: 'Information' },
    { value: 'poll', label: 'Vote' },
  ];

  const tags = [
    {
      value: 'FacInfo',
      label: (
        <span>
          <FaTag /> FacInfo
        </span>
      ),
    },
    {
      value: 'FacEco',
      label: (
        <span>
          <FaTag /> FacEco
        </span>
      ),
    },
    {
      value: 'Arsenal',
      label: (
        <span>
          <FaTag /> Arsenal
        </span>
      ),
    },
  ];

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
            options={tags}
            isMulti
            placeholder={'Sélectionner un ou plusieurs tags'}
            styles={customStyles}
          />
          <br />
          <Form.Group>
            <Form.Control as="textarea" rows="5" placeholder="Text.." />
          </Form.Group>
          {category == 'poll' && <PoolSection />}

          <Button variant="primary" className="mt-1 float-right">
            Créer
          </Button>
        </Form>
      </Card.Body>
    </Card>
  );
}

function PoolSection() {
  const [pollOptions, setPollOptions] = useState([1, 2]);

  function addPoolOption() {
    setPollOptions(pollOptions.concat([pollOptions.length + 1]));
  }
  function removePoolOption() {
    setPollOptions([...pollOptions].slice(0, -1));
  }
  return (
    <>
      <Form.Group>
        {pollOptions.map((i) => (
          <>
            {i > 2 ? (
              <>
                <Form.Control type="text" placeholder={'Option ' + i} />
                <TiDelete size={20} onClick={removePoolOption} />
              </>
            ) : (
              <Form.Control type="text" placeholder={'Option ' + i} />
            )}

            <br />
          </>
        ))}

        {pollOptions.length < 5 && (
          <a href="#" onClick={addPoolOption}>
            <FaPlusSquare className="mr-1" size={20} />
            Ajouter une option
          </a>
        )}
      </Form.Group>
    </>
  );
}

About.defaultProps = {};

export default About;
