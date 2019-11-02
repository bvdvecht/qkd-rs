
# Create new qubit

## Request

#### CQC Header
- `Command`
- *version,  app_id,  length*

#### CQC Command Header
- *qubit_id* (not used)
- `New`
- *options*

  

## Response

#### CQC Header
- `NewOk`
- *version,  app_id,  length*

#### CQC Extra Qubit Header
- *id  of  created  qubit*

#### [ if Notify option is set ] CQC Header
- `Done`
- *version,  app_id,  length*

  
  

# Send qubit

## Request

#### CQC Header

- `Command`
- *version,  app_id,  length*

#### CQC Command Header

- *qubit_id  of  qubit  to  be  sent*
- `Send`
- *options*

#### CQC Communication Header

- *remote_app_id*
- *remote_port*
- *remote_node*

  

## Response


#### [ if Notify option is set ] CQC Header
- `Done`
- *version,  app_id,  length*

  
  
  

# Receive qubit

## Request


#### CQC Header

- `Command`
- *version,  app_id,  length*


#### CQC Command Header
- *qubit_id* (not used)
- `Recv`
- *options*

  

## Response

#### CQC Header
- `Recv`
- *version,  app_id,  length*

#### CQC Extra Qubit Header
- *id  of  received  qubit*


#### [ if Notify option is set ] CQC Header
- `Done`
- *version,  app_id,  length*
  
  
  

# Measure qubit

## Request


#### CQC Header

- `Command`
- *version,  app_id,  length*

#### CQC Command Header
- *qubit_id  of  qubit  to  be  measured*
- `Measure`
- *options*  

## Response

#### CQC Header
- `MeasOut`
- *version,  app_id,  length*

#### CQC Measurement Outcome Header
- *meas_out*


#### [ if Notify option is set ] CQC Header
- `Done`
- *version,  app_id,  length*

  
  
  

# Create EPR

## Request


#### CQC Header

- `Command`
- *version,  app_id,  length*

#### CQC Command Header
- *qubit_id* (not used)
- `Epr`
- *options*


#### CQC Communication Header

- *remote_app_id*
- *remote_port*
- *remote_node*

  

## Response

#### CQC Header
- `EprOk`
- *version,  app_id,  length*

#### CQC Entanglement Information Header
- *entanglement  info*


#### [ if Notify option is set ] CQC Header
- `Done`
- *version,  app_id,  length*

  
  

# Receive EPR

## Request


#### CQC Header
- `Command`
- *version,  app_id,  length*

#### CQC Command Header
- *qubit_id* (not used)
- `EprRecv`
- *options*

  
## Response

#### CQC Header
- `EprOk`
- *version,  app_id,  length*

#### CQC Entanglement Information Header
- *entanglement  info*


#### [ if Notify option is set ] CQC Header
- `Done`
- *version,  app_id,  length*

  
  
  

# Apply H gate

## Request


#### CQC Header
- `Command`
- *version,  app_id,  length*

#### CQC Command Header
- *qubit_id  of  qubit  to  apply  Hadamard  on*
- `H`
- *options*

  
## Response


#### [ if Notify option is set ] CQC Header
- `Done`
- *version,  app_id,  length*
