import type { Meta, StoryObj } from '@storybook/react'

import { DiffViewer } from './DiffViewer'
import { withRouter } from 'storybook-addon-react-router-v6'
import { type TableDiff } from '../../../types'

const meta = {
  title: 'Templates/DiffViewer',
  component: DiffViewer,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof DiffViewer>

export default meta
type Story = StoryObj<typeof meta>

const object: (x: any[]) => Record<any, any> = (x) => Object.fromEntries(new Map(x))

const create: (tablesN: number, primaryValuesN: number, colsN: number) => TableDiff[] = (tablesN, primaryValuesN, colsN) => {
  const tables = [...Array(tablesN).keys()]
  const primaryValues = [...Array(primaryValuesN).keys()].map((i) => `${i}`)
  const cols = [...Array(colsN).keys()]

  return tables.map((table) => ({
    tableName: `table-${table}`,
    primaryValues,
    primaryColName: 'id',
    colNames: cols.map((col) => `col-${col}`),
    rowDiffs1: object(
      primaryValues.map((primaryValue) => [
        primaryValue,
        object(
          cols.map((col) => [
            `col-${col}`,
            {
              status: 'deleted',
              value: `prev-${col}`,
            },
          ])
        ),
      ])
    ),
    rowDiffs2: object(
      primaryValues.map((primaryValue) => [
        primaryValue,
        object(
          cols.map((col) => [
            `col-${col}`,
            {
              status: 'added',
              value: `current-${col}`,
            },
          ])
        ),
      ])
    ),
  }))
}

export const Small: Story = {
  args: {
    tableDiffs: create(5, 3, 5),
  },
}

export const Medium: Story = {
  args: {
    tableDiffs: create(5, 10, 10),
  },
}

export const Large: Story = {
  args: {
    tableDiffs: create(10, 25, 20),
  },
}

export const Huge: Story = {
  args: {
    tableDiffs: create(3, 1000, 5),
  },
}

export const Empty: Story = {
  args: {
    tableDiffs: [],
  },
}
