const Categories = ({categories}) => {
    return (
        <div>
          <h1>Categories:</h1>
          <ul>
            {categories.map((cat) => (
              <li key={cat.id}>
                {cat.name}: {cat.budget != null ? `$${cat.budget}` : '--'}
              </li>
            ))}
          </ul>
        </div>
      );
};

export default Categories;
