import type { Meta, StoryObj } from '@storybook/react'

import { InputText } from './InputText'

const meta = {
  title: 'Atoms/InputText',
  component: InputText,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof InputText>

export default meta
type Story = StoryObj<typeof meta>

export const Empty: Story = {
  args: {
    maxLength: 10,
    onChange: console.log,
    chars: 'all',
  },
}

export const HalfWithMax: Story = {
  args: {
    value: '0123456789',
    maxLength: 10,
    onChange: console.log,
    chars: 'all',
  },
}

export const FullWithMax: Story = {
  args: {
    value: 'あいうえおかきくけこ',
    maxLength: 10,
    onChange: console.log,
    chars: 'all',
  },
}
