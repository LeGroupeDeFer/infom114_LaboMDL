import React from 'react';
import { Container, Row, Col } from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faTag, faSearch } from '@fortawesome/free-solid-svg-icons';
import { components } from 'react-select';
import CreatableSelect from 'react-select/creatable';


const DropdownIndicator = (props) => {
  return (
    <components.DropdownIndicator {...props}>
      <Icon icon={faSearch} />
    </components.DropdownIndicator>
  );
};

// SearchBar :: None => Component

function SearchOption({ icon, label }) {
  return (
    <div className="search-option">
      <span className="search-option-icon"><Icon icon={icon}/></span>
      <span className="search-option-label">{label}</span>
    </div>
  );
}

function SearchBar({ tags, onChange, children }) {

  const options = [
    {
      value: 'FacInfo',
      label: <SearchOption icon={faTag} label="FacInfo" />
    },
    {
      value: 'FacEco',
      label: <SearchOption icon={faTag} label="FacEco" />
    },
    {
      value: 'Arsenal',
      label: <SearchOption icon={faTag} label="Arsenal" />
    },
  ];

  return (
    <Container fluid className="search-container py-2">
      <Row>

        <Col md={4}>
          <CreatableSelect
            id="search-bar"
            className="search-bar my-1"
            classNamePrefix="search-bar"
            isMulti
            options={options}
            components={{ DropdownIndicator }}
            placeholder="Rechercher"
            formatCreateLabel={(userInput) => `Rechercher "${userInput}"`}
            value={tags}
            onChange={onChange}
          />
        </Col>

        <Col md={4}>{children}</Col>

      </Row>
    </Container>
  );
}

export default SearchBar;