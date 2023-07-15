import type { Meta, StoryObj } from '@storybook/react'

import { DiffViewer } from './DiffViewer'
import { type TableDiff } from '../../../types'
import { withRouter } from 'storybook-addon-react-router-v6'

const meta = {
  title: 'Templates/DiffViewer',
  component: DiffViewer,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof DiffViewer>

export default meta
type Story = StoryObj<typeof meta>

const tableDiff: Omit<TableDiff, 'tableName'> = {
  primaryValues: ['1', '2'],
  primaryColName: 'id',
  colNames: ['name', 'age'],
  rowDiffs1: {
    '1': {
      name: { status: 'deleted', value: '"John"' },
      age: { status: 'deleted', value: '29' },
    },
    '2': {
      name: { status: 'deleted', value: '"Alice"' },
      age: { status: 'deleted', value: '31' },
    },
  },
  rowDiffs2: {
    '1': {
      name: { status: 'added', value: '"Jane"' },
      age: { status: 'added', value: '15' },
    },
  },
}

const tableNames = [
  'actions',
  'administrators',
  'bills',
  'campaigns',
  'cards',
  'emails',
  'events',
  'friends',
  'groups',
  'items',
  'letters',
  'locks',
  'login_histories',
  'mails',
  'messages',
  'options',
  'payments',
  'plans',
  'profiles',
  'receipts',
  'roles',
  'shipments',
  'users',
]

export const Component: Story = {
  args: {
    tableDiffs: tableNames.map((tableName) => ({ tableName, ...tableDiff })),
    ignoreTableNames: [],
  },
}
