# Simple Payment Engine

This is a simple payment engine capable of processing transactions ingested via a csv input file

## Usage

`cargo run --release -- path/to/csv > accounts.csv`

## Input

### Data types

| field  | type |
|--------|------|
| client | u16  |
| tx     | u32  |
| amount | f32  |

#### Example input:

```csv
type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0
```

### Valid Transaction Types

#### Deposit

A deposit is a credit to the client's asset account, meaning it should increase the available and
total funds of the client account

example:

| type    | client | tx  | amount |
|---------|--------|-----|--------|
| deposit | 1      | 1   | 1.0    |

#### Withdrawal

A withdraw is a debit to the client's asset account, meaning it should decrease the available and
total funds of the client account

example:

| type       | client | tx  | amount |
|------------|--------|-----|--------|
| withdrawal | 1      | 1   | 1.0    |

#### Dispute

A dispute represents a client's claim that a transaction was erroneous and should be reversed.
The transaction shouldn't be reversed yet but the associated funds should be held. This means
that the clients available funds should decrease by the amount disputed, their held funds should
increase by the amount disputed, while their total funds should remain the same

example:

| type    | client | tx  | amount |
|---------|--------|-----|--------|
| dispute | 1      | 1   |        |

#### Resolve

A resolve represents a resolution to a dispute, releasing the associated held funds. Funds that
were previously disputed are no longer disputed. This means that the clients held funds should
decrease by the amount no longer disputed, their available funds should increase by the
amount no longer disputed, and their total funds should remain the same

example:

| type    | client | tx  | amount |
|---------|--------|-----|--------|
| resolve | 1      | 1   |        |

#### Chargeback

A chargeback is the final state of a dispute and represents the client reversing a transaction.
Funds that were held have now been withdrawn. This means that the clients held funds and
total funds should decrease by the amount previously disputed. If a chargeback occurs the
client's account should be immediately frozen

example:

| type       | client | tx  | amount |
|------------|--------|-----|--------|
| chargeback | 1      | 1   |        |

## Output

| column    | description                                                                                  |
|-----------|----------------------------------------------------------------------------------------------|
| available | The total funds that are available. This should be equal to the total - held amounts         |
| held      | The total funds that are held for dispute. This should be equal to total - available amounts |
| total     | The total funds that are available or held. This should be equal to available + held         |
| locked    | Whether the account is locked. An account is locked if a charge back occurs                  |

example:

```csv
client, available, held, total, locked
1, 1.5, 0.0, 1.5, false
2, 2.0, 0.0, 2.0, false
```

## Rules

### Withdrawal

If a client does not have sufficient available funds the withdrawal should fail and the total amount
of funds should not change

### Dispute

A dispute references the transaction that is disputed by ID. If the tx specified by the dispute doesn't exist, it 
will be ignored

### Resolve

A resolve references the transaction by ID. If the tx specified by the resolve doesn't exist or is not disputed, it
will be ignored

### Chargeback

A chargeback references the transaction by ID. If the tx specified by the chargeback doesn't exist or is not 
disputed, it
will be ignored
