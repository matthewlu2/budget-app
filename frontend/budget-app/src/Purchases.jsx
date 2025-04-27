const Purchases = ({purchases}) => {
    return (
        <div>
          <h1>Purchases:</h1>
          <ul>
            {purchases.map((pur) => (
              <li key={pur.id}>
                [{pur.date}] ${pur.amount} {pur.desc} ({pur.category})
              </li>
            ))}
          </ul>
        </div>
      );
};

export default Purchases;
